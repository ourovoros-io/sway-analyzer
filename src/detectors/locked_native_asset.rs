use crate::{
    error::Error,
    project::Project,
    report::Severity,
    scope::AstScope,
    utils,
    visitor::{AsmBlockContext, AstVisitor, ExprContext, FnContext, ModuleContext},
};
use std::{cell::RefCell, collections::HashMap, path::PathBuf, rc::Rc};
use sway_types::{Span, Spanned};

#[derive(Default)]
pub struct LockedNativeAssetVisitor {
    module_states: HashMap<PathBuf, ModuleState>,
}

struct ModuleState {
    fn_calls_to_check: Vec<String>,
    locking_functions: Vec<(Span, String)>,
    can_receive: bool,
    has_withdraw: bool,
}

impl Default for ModuleState {
    fn default() -> Self {
        Self {
            fn_calls_to_check: vec![
                "std::asset::transfer".into(),
                "std::asset::transfer_to_address".into(),
                "std::asset::force_transfer_to_contract".into(),
                "std::low_level_call::call_with_function_selector".into(),
            ],
            locking_functions: vec![],
            can_receive: false,
            has_withdraw: false,
        }
    }
}

impl AstVisitor for LockedNativeAssetVisitor {
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

        for attr in context.fn_attributes {
            if attr.span().as_str().contains("payable") {
                module_state.can_receive = true;
                
                module_state.locking_functions.push((
                    context.item_fn.fn_signature.span(),
                    utils::get_item_location(context.item, &context.item_impl, &Some(context.item_fn)),
                ));
            }
        }

        Ok(())
    }

    fn visit_expr(&mut self, context: &ExprContext, _scope: Rc<RefCell<AstScope>>, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();
        
        // Only check function applications
        let sway_ast::Expr::FuncApp { func, .. } = &context.expr else { return Ok(()) };

        if module_state.fn_calls_to_check.iter().any(|x| x.contains(func.span().as_str())) {
            module_state.has_withdraw = true;
        }

        Ok(())
    }

    fn visit_asm_block(&mut self, context: &AsmBlockContext, _scope: Rc<RefCell<AstScope>>, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();
        
        for (instr, _) in &context.asm.contents.inner.instructions {
            // TODO: check if the function is a withdraw function
            let sway_ast::Instruction::Call { .. } = instr else { continue };
            module_state.has_withdraw = true;
        }

        Ok(())
    }

    fn leave_module(&mut self, context: &ModuleContext, _scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();
        
        if module_state.can_receive && !module_state.has_withdraw {
            for (function_span, function_name) in module_state.locking_functions.iter() {
                project.report.borrow_mut().add_entry(
                    context.path,
                    project.span_to_line(context.path, function_span)?,
                    Severity::High,
                    format!(
                        "{} will lock native assets. Consider adding a withdraw function.",
                        function_name,
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
    fn test_locked_native_asset() {
        crate::tests::test_detector("locked_native_asset", 2);
    }
}
