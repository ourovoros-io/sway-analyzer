use crate::{
    error::Error,
    project::Project,
    report::Severity,
    scope::AstScope,
    utils,
    visitor::{AstVisitor, ExprContext, FnContext, ModuleContext, StatementLetContext},
};
use std::{cell::RefCell, collections::HashMap, path::PathBuf, rc::Rc};
use sway_ast::Expr;
use sway_types::{Span, Spanned};

#[derive(Default)]
pub struct StrictEqualityVisitor {
    module_states: HashMap<PathBuf, ModuleState>,
}

#[derive(Default)]
struct ModuleState {
    fn_states: HashMap<Span, FnState>,
}

#[derive(Default)]
struct FnState {
    balance_vars: Vec<String>,
}

impl AstVisitor for StrictEqualityVisitor {
    fn visit_module(&mut self, context: &ModuleContext, _scope: Rc<RefCell<AstScope>>, _project: &mut Project) -> Result<(), Error> {
        // Create the module state
        if !self.module_states.contains_key(context.path) {
            self.module_states
                .insert(context.path.into(), ModuleState::default());
        }

        Ok(())
    }

    fn visit_fn(&mut self, context: &FnContext, _scope: Rc<RefCell<AstScope>>, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Create the function state
        let fn_signature = context.item_fn.fn_signature.span();
        
        module_state.fn_states.entry(fn_signature).or_default();

        Ok(())
    }

    fn visit_statement_let(&mut self, context: &StatementLetContext, _scope: Rc<RefCell<AstScope>>, _project: &mut Project) -> Result<(), Error> {
        //  Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get_mut(&fn_signature).unwrap();

        if context.statement_let.expr.span().as_str().contains("balance") {
            fn_state.balance_vars.push(context.statement_let.pattern.span().str());
        }

        Ok(())
    }

    fn visit_expr(&mut self, context: &ExprContext, _scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        //  Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Get the function state
        let Some(item_fn) = context.item_fn.as_ref() else { return Ok(()) };
        let fn_signature = item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get_mut(&fn_signature).unwrap();

        if !context.expr.span().as_str().contains("balance") && !fn_state.balance_vars.iter().any(|x| context.expr.span().as_str().contains(x)) {
            return Ok(());
        }

        let sway_ast::Expr::Equal { lhs, rhs, .. } = context.expr else { return Ok(()) };

        if matches!(lhs.as_ref(), Expr::Literal(_)) || matches!(rhs.as_ref(), Expr::Literal(_)) {
            project.report.borrow_mut().add_entry(context.path,
                project.span_to_line(context.path, &context.expr.span())?,
                Severity::High,
                format!(
                    "{} contains a strict equality check: `{}`. Don't use strict equality to determine if an account has enough balance.",
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
    fn test_strict_equality() {
        crate::tests::test_detector("strict_equality", 2);
    }
}
