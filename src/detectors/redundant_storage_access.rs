use crate::{
    error::Error,
    project::Project,
    report::Severity,
    utils,
    visitor::{
        AstVisitor, BlockContext, FnContext, ModuleContext, StatementContext, WhileExprContext,
    },
};
use std::{collections::HashMap, path::PathBuf};
use sway_ast::Statement;
use sway_types::{Span, Spanned};

#[derive(Default)]
pub struct RedundantStorageAccessVisitor {
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
    storage_reads: Vec<Span>,
    storage_writes: Vec<Span>,
}

impl AstVisitor for RedundantStorageAccessVisitor {
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

    fn visit_while_expr(&mut self, context: &WhileExprContext, project: &mut Project) -> Result<(), Error> {
        // Check if the loop's condition contains redundant storage access
        if let Some(expr) = utils::find_storage_access_in_expr(context.condition) {
            project.report.borrow_mut().add_entry(
                context.path,
                project.span_to_line(context.path, &expr.span())?,
                Severity::Low,
                format!(
                    "{} contains a loop condition with redundant storage access: `{}`. Consider storing the value in a local variable in order to lower gas costs.",
                    utils::get_item_location(context.item, &context.item_impl, &Some(context.item_fn)),
                    expr.span().as_str(),
                ),
            );
        }

        Ok(())
    }

    fn visit_statement(&mut self, context: &StatementContext, project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();
        
        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get_mut(&fn_signature).unwrap();

        // Check if the statement contains storage access
        match context.statement {
            Statement::Let(stmt_let) => {
                let expr = utils::find_storage_access_in_expr(&stmt_let.expr);
                let Some(expr) = expr.as_ref() else { return Ok(()) };
                
                let idents = utils::fold_expr_idents(expr);
                let "read" = idents.last().unwrap().as_str() else { return Ok(()) };

                for block_span in context.blocks.iter().rev() {
                    let block_state = fn_state.block_states.get(block_span).unwrap();

                    if block_state.storage_reads.iter().any(|x| x.as_str() == idents[1].as_str()) {
                        project.report.borrow_mut().add_entry(
                            context.path,
                            project.span_to_line(context.path, &expr.span())?,
                            Severity::Low,
                            format!(
                                "{} contains a redundant storage access: `{}`. Consider storing the value in a local variable in order to lower gas costs.",
                                utils::get_item_location(context.item, &context.item_impl, &Some(context.item_fn)),
                                expr.span().as_str(),
                            ),
                        );
                        break;
                    }
                }

                // Update the block state
                let block_span = context.blocks.last().unwrap();
                let block_state = fn_state.block_states.get_mut(block_span).unwrap();
                block_state.storage_reads.push(idents[1].span());
            }
            
            Statement::Expr { expr, .. } => {
                let expr = utils::find_storage_access_in_expr(expr);
                let Some(expr) = expr.as_ref() else { return Ok(()) };
                
                let idents = utils::fold_expr_idents(expr);
                let "write" = idents.last().unwrap().as_str() else { return Ok(()) };

                for block_span in context.blocks.iter().rev() {
                    let block_state = fn_state.block_states.get(block_span).unwrap();

                    if block_state.storage_writes.iter().any(|x| x.as_str() == idents[1].as_str()) {
                        project.report.borrow_mut().add_entry(
                            context.path,
                            project.span_to_line(context.path, &expr.span())?,
                            Severity::Low,
                            format!(
                                "{} contains a redundant storage update: `{}`. Consider limiting to a single storage write in order to lower gas costs.",
                                utils::get_item_location(context.item, &context.item_impl, &Some(context.item_fn)),
                                expr.span().as_str(),
                            ),
                        );
                        break;
                    }
                }

                // Update the block state
                let block_span = context.blocks.last().unwrap();
                let block_state = fn_state.block_states.get_mut(block_span).unwrap();
                block_state.storage_writes.push(idents[1].span());
            }
            
            _ => {}
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_redundant_storage_access() {
        crate::tests::test_detector("redundant_storage_access", 5);
    }
}
