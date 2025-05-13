use crate::{
    error::Error,
    project::Project,
    report::Severity,
    scope::AstScope,
    utils,
    visitor::{AstVisitor, ExprContext, FnContext, ModuleContext},
};
use std::{cell::RefCell, collections::HashMap, path::PathBuf, rc::Rc};
use sway_types::{Span, Spanned};

#[derive(Default)]
pub struct UnprotectedInitializationVisitor {
    module_states: HashMap<PathBuf, ModuleState>,
}

#[derive(Default)]
struct ModuleState {
    fn_states: HashMap<String, FnState>,
}

#[derive(Clone)]
struct FnState {
    is_init_fn: bool,
    has_requirement: bool,
    finalized: bool,
    location: String,
    span: Span,
    function_calls: Vec<String>,
}

impl AstVisitor for UnprotectedInitializationVisitor {
    fn visit_module(&mut self, context: &ModuleContext, _scope: Rc<RefCell<AstScope>>, _project: &mut Project) -> Result<(), Error> {
        // Create the module state
        if !self.module_states.contains_key(context.path) {
            self.module_states.insert(context.path.into(), ModuleState::default());
        }

        Ok(())
    }

    fn visit_fn(&mut self, context: &FnContext, _scope: Rc<RefCell<AstScope>>, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Create the function state
        let fn_signature = context.item_fn.fn_signature.name.to_string();

        module_state.fn_states.entry(fn_signature).or_insert_with(|| FnState {
            is_init_fn: context.item_fn.fn_signature.name.as_str().contains("init"),
            has_requirement: false,
            finalized: false,
            location: utils::get_item_location(context.item, &context.item_impl, &Some(context.item_fn)),
            span: context.item_fn.fn_signature.span(),
            function_calls: vec![],
        });

        Ok(())
    }

    fn visit_expr(&mut self, context: &ExprContext, _scope: Rc<RefCell<AstScope>>, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Get the function state
        let Some(item_fn) = context.item_fn.as_ref() else { return Ok(()); };
        let fn_signature = item_fn.fn_signature.name.to_string();
        let fn_state = module_state.fn_states.get_mut(&fn_signature).unwrap();

        // Check for `require` and update the function state
        if utils::get_require_args(context.expr).is_some() || utils::get_if_revert_condition(context.expr).is_some() {
            fn_state.has_requirement = true;
        }

        // Check for function calls
        if let sway_ast::Expr::FuncApp { func, .. } = context.expr {
            if let sway_ast::Expr::Path(_) = func.as_ref() {
                let func = func.span().as_str().to_string();
                if !fn_state.function_calls.contains(&func) {
                    fn_state.function_calls.push(func);
                }
            }
        }

        Ok(())
    }

    fn leave_module(&mut self, context: &ModuleContext, _scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        fn finalize_fn_state(mut fn_states: HashMap<String, FnState>, fn_state: &mut FnState) {
            if fn_state.finalized {
                return;
            }

            for function_call in fn_state.function_calls.iter() {
                let Some((_, mut called_fn_state)) = fn_states.iter_mut().find(|f| f.0.as_str() == function_call).map(|(a, b)| (a.clone(), b.clone())) else { continue };
                if !called_fn_state.finalized {
                    finalize_fn_state(fn_states.clone(), &mut called_fn_state);
                }

                if called_fn_state.has_requirement {
                    fn_state.has_requirement = true;
                    break;
                }
            }
        }

        let temp_fn_states = module_state.fn_states.clone();

        for (_, fn_state) in module_state.fn_states.iter_mut() {
            finalize_fn_state(temp_fn_states.clone(), fn_state);
            
            if fn_state.is_init_fn && !fn_state.has_requirement {
                project.report.borrow_mut().add_entry(
                    context.path,
                    project.span_to_line(context.path, &fn_state.span)?,
                    Severity::High,
                    format!(
                        "{} is an unprotected initializer function. Consider adding a requirement to prevent it from being called multiple times.",
                        fn_state.location,
                    ),
                );
            }
        }        

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_unprotected_initialization() {
        crate::tests::test_detector("unprotected_initialization", 1);
    }
}
