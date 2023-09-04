use crate::{
    error::Error,
    project::Project,
    report::Severity,
    visitor::{AstVisitor, BlockContext, ExprContext, FnContext, ModuleContext, StatementContext}, utils,
};
use std::{collections::HashMap, path::PathBuf};
use sway_ast::{Expr, Statement, StatementLet};
use sway_types::{Span, Spanned};

#[derive(Default)]
pub struct DivisionBeforeMultiplicationVisitor {
    module_states: HashMap<PathBuf, ModuleState>,
}

#[derive(Default)]
struct ModuleState {
    fn_states: HashMap<Span, FnState>,
}

#[derive(Default)]
struct FnState {
    block_states: HashMap<Span, BlockState>,
}

#[derive(Default)]
struct BlockState {
    variable_has_division: HashMap<Span, bool>,
}

impl AstVisitor for DivisionBeforeMultiplicationVisitor {
    fn visit_module(&mut self, context: &ModuleContext, _project: &mut Project) -> Result<(), Error> {
        // Create the module state
        if !self.module_states.contains_key(context.path) {
            self.module_states.insert(context.path.into(), ModuleState::default());
        }

        Ok(())
    }

    fn visit_fn(&mut self, context: &FnContext, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Create the function state
        let fn_signature = context.item_fn.fn_signature.span();
        
        if !module_state.fn_states.contains_key(&fn_signature) {
            module_state.fn_states.insert(fn_signature, FnState::default());
        }
        
        Ok(())
    }

    fn visit_block(&mut self, context: &BlockContext, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get_mut(&fn_signature).unwrap();

        // Create the block state
        let block_span = context.block.span();

        if !fn_state.block_states.contains_key(&block_span) {
            fn_state.block_states.insert(block_span, BlockState::default());
        }
        
        Ok(())
    }

    fn visit_statement(&mut self, context: &StatementContext, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get_mut(&fn_signature).unwrap();

        // Get the block state
        let block_span = context.blocks.last().unwrap();
        let block_state = fn_state.block_states.get_mut(block_span).unwrap();

        // Check if the statement declares a variable which stores the result of a division
        if let Statement::Let(StatementLet { pattern, expr: Expr::Div { .. }, .. }) = context.statement {
            block_state.variable_has_division.insert(pattern.span(), true);
        }

        Ok(())
    }

    fn visit_expr(&mut self, context: &ExprContext, project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get(context.path).unwrap();

        // Only check multiplication expressions
        let Expr::Mul { lhs, .. } = context.expr else { return Ok(()) };

        fn check_for_division(expr: &Expr) -> bool {
            match expr {
                Expr::Parens(expr) => check_for_division(expr.inner.as_ref()),
                Expr::Div { .. } => true,
                _ => false,
            }
        }

        let mut has_division = false;

        if let Some(item_fn) = context.item_fn.as_ref() {
            // Get the function state
            let fn_signature = item_fn.fn_signature.span();
            let fn_state = module_state.fn_states.get(&fn_signature).unwrap();
    
            // Check if `lhs` is a variable in scope that stores a division
            'var_lookup: for block_span in context.blocks.iter().rev() {
                let block_state = fn_state.block_states.get(block_span).unwrap();
    
                for (k, variable_has_division) in block_state.variable_has_division.iter() {
                    if k.as_str() == lhs.span().as_str() {
                        has_division = *variable_has_division;
                        break 'var_lookup;
                    }
                }
            }
        }

        if !has_division {
            has_division = check_for_division(lhs.as_ref());
        }

        if !has_division {
            return Ok(());
        }

        project.report.borrow_mut().add_entry(
            context.path,
            project.span_to_line(context.path, &context.expr.span())?,
            Severity::Low,
            format!(
                "{} contains a multiplication on the result of a division, which can truncate: `{}`. Consider refactoring in order to prevent value truncation.",
                utils::get_item_location(context.item, &context.item_impl, &context.item_fn),
                context.expr.span().as_str(),
            ),
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_division_before_multiplication() {
        crate::tests::test_detector("division_before_multiplication")
    }
}
