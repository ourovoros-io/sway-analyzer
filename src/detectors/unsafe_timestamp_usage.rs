use crate::{
    error::Error,
    project::Project,
    report::Severity,
    scope::AstScope,
    utils,
    visitor::{AstVisitor, ExprContext, ModuleContext, UseContext},
};
use std::{cell::RefCell, collections::HashMap, path::PathBuf, rc::Rc};
use sway_ast::Expr;
use sway_types::Spanned;

#[derive(Default)]
pub struct UnsafeTimestampUsageVisitor {
    module_states: HashMap<PathBuf, ModuleState>,
}

#[derive(Default)]
struct ModuleState {
    timestamp_names: Vec<String>,
    timestamp_of_block_names: Vec<String>,
}

impl ModuleState {
    fn expr_is_timestamp_call(&self, expr: &Expr) -> bool {
        // Destructure the expression into a function application
        let Expr::FuncApp { func, .. } = expr else { return false };
        let Expr::Path(path) = func.as_ref() else { return false };
        
        // Update the variable state
        match path.span().as_str() {
            "std::block::timestamp" => true,
            "std::block::timestamp_of_block" => true,
            s if self.timestamp_names.iter().any(|name| s == name) => true,
            s if self.timestamp_of_block_names.iter().any(|name| s == name) => true,
            _ => false,
        }
    }
}

impl AstVisitor for UnsafeTimestampUsageVisitor {
    fn visit_module(&mut self, context: &ModuleContext, _scope: Rc<RefCell<AstScope>>, _project: &mut Project) -> Result<(), Error> {
        // Create the module state
        if !self.module_states.contains_key(context.path) {
            self.module_states.insert(context.path.into(), ModuleState::default());
        }

        Ok(())
    }

    fn visit_use(&mut self, context: &UseContext, _scope: Rc<RefCell<AstScope>>, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Check the use tree for `std::block::timestamp`
        if let Some(name) = utils::use_tree_to_name(&context.item_use.tree, "std::block::timestamp") {
            module_state.timestamp_names.push(name);
        }
        
        // Check the use tree for `std::block::timestamp_of_block`
        if let Some(name) = utils::use_tree_to_name(&context.item_use.tree, "std::block::timestamp_of_block") {
            module_state.timestamp_of_block_names.push(name);
        }

        Ok(())
    }

    fn visit_expr(&mut self, context: &ExprContext, _scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Check if the expression is a timestamp call expression
        if module_state.expr_is_timestamp_call(context.expr) {
            project.report.borrow_mut().add_entry(
                context.path,
                project.span_to_line(context.path, &context.expr.span())?,
                Severity::Medium,
                format!(
                    "{} contains dependence on a block timestamp, which can be manipulated by an attacker: `{}`",
                    utils::get_item_location(context.item, &context.item_impl, &context.item_fn),
                    context.expr.span().as_str(),
                ),
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_unsafe_timestamp_usage() {
        crate::tests::test_detector("unsafe_timestamp_usage", 6);
    }
}
