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
    fn_states: HashMap<Span, FnState>,
}

#[derive(Default)]
struct FnState {
    is_init_fn: bool,
    has_requirement: bool,
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
        let fn_signature = context.item_fn.fn_signature.span();

        module_state.fn_states.entry(fn_signature).or_insert_with(|| FnState {
            is_init_fn: context.item_fn.fn_signature.name.as_str().contains("init"),
            ..Default::default()
        });

        Ok(())
    }

    fn leave_fn(&mut self, context: &FnContext, _scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get(context.path).unwrap();

        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get(&fn_signature).unwrap();

        if fn_state.is_init_fn && !fn_state.has_requirement {
            project.report.borrow_mut().add_entry(
                context.path,
                project.span_to_line(context.path, &fn_signature)?,
                Severity::High,
                format!(
                    "{} is an unprotected initializer function. Consider adding a requirement to prevent it from being called multiple times.",
                    utils::get_item_location(context.item, &context.item_impl, &Some(context.item_fn)),
                ),
            );
        }

        Ok(())
    }

    fn visit_expr(&mut self, context: &ExprContext, _scope: Rc<RefCell<AstScope>>, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Get the function state
        let Some(item_fn) = context.item_fn.as_ref() else { return Ok(()); };
        let fn_signature = item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get_mut(&fn_signature).unwrap();

        // Check for `require` and update the function state
        if utils::get_require_args(context.expr).is_some() || utils::get_if_revert_condition(context.expr).is_some() {
            fn_state.has_requirement = true;
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
