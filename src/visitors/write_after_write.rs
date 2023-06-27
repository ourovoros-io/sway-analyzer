use super::{AstVisitor, BlockContext, ExprContext, FnContext, ModuleContext, StatementContext, WhileExprContext};
use crate::{error::Error, project::Project, utils};
use std::{collections::HashMap, path::PathBuf};
use sway_types::{Span, Spanned};

#[derive(Default)]
pub struct WriteAfterWriteVisitor {
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
    var_states: HashMap<Span, VarState>,
}

#[derive(Default)]
struct VarState {
    name: String,
    modified_span: Option<Span>,
    used: bool,
}

impl AstVisitor for WriteAfterWriteVisitor {
    fn visit_module(&mut self, context: &ModuleContext,  _project: &mut Project) -> Result<(), Error> {
        // Create the module state
        if !self.module_states.contains_key(context.path) {
            self.module_states.insert(context.path.into(), ModuleState::default());
        }

        Ok(())
    }

    fn visit_fn(&mut self, context: &FnContext,  _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Create the function state
        let fn_signature = context.item_fn.fn_signature.span();

        if !module_state.fn_states.contains_key(&fn_signature) {
            module_state.fn_states.insert(fn_signature, FnState::default());
        }

        Ok(())
    }

    fn visit_block(&mut self, context: &BlockContext,  _project: &mut Project) -> Result<(), Error> {
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

    fn leave_block(&mut self, context: &BlockContext, project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get(context.path.into()).unwrap();

        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get(&fn_signature).unwrap();

        // Get the block state
        let block_span = context.block.span();
        let block_state = fn_state.block_states.get(&block_span).unwrap();

        for (var_span, var_state) in block_state.var_states.iter() {
            if let Some(modified_span) = var_state.modified_span.as_ref() {
                //
                // TODO: check state of specific fields/tuple members
                //

                if !var_state.used {
                    project.report.borrow_mut().add_entry(
                        context.path,
                        project.span_to_line(context.path, modified_span)?,
                        format!("Variable `{}` consecutively modified without being utilized.", var_span.as_str()),
                    );
                }
            }
        }

        Ok(())
    }

    fn leave_while_expr(&mut self, context: &WhileExprContext, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get_mut(&fn_signature).unwrap();

        // Collect all identifier spans in `context.condition`
        let var_spans = utils::collect_ident_spans(context.condition);

        // Find the block state each variable state was declared in
        for var_span in var_spans {
            for block_span in context.blocks.iter().rev() {
                // Get the block state
                let block_state = fn_state.block_states.get_mut(&block_span).unwrap();
        
                // Find the variable state and mark it as used
                if let Some((_, var_state)) = block_state.var_states.iter_mut().find(|(_, var_state)| var_state.name == var_span.as_str()) {
                    var_state.used = true;
                    break;
                }
            }
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
        let block_state = fn_state.block_states.get_mut(&block_span).unwrap();

        // Check if we have encountered a variable declaration
        if let Some(var_ident) = utils::statement_to_variable_binding_ident(context.statement) {
            // Create the variable state
            let var_span = var_ident.span();

            if !block_state.var_states.contains_key(&var_span) {
                block_state.var_states.insert(var_span.clone(), VarState {
                    name: var_span.as_str().to_string(),
                    ..Default::default()
                });
            }
        }
        // Check if we have encountered a variable reassignment
        else if let Some(reassignment_idents) = utils::statement_to_reassignment_idents(context.statement) {
            let var_span = reassignment_idents.first().unwrap().span();
            let var_name = var_span.as_str().to_string();

            for block_span in context.blocks.iter().rev() {
                let block_state = fn_state.block_states.get_mut(block_span).unwrap();

                if let Some((_, var_state)) = block_state.var_states.iter_mut().find(|(_, var_state)| var_state.name == var_name) {
                    var_state.modified_span = Some(var_span);
                    var_state.used = false;

                    //
                    // TODO: update state of specific fields/tuple members
                    //

                    break;
                }
            }
        }

        Ok(())
    }

    fn visit_expr(&mut self, context: &ExprContext, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get_mut(&fn_signature).unwrap();

        let expr_idents = utils::fold_expr_idents(context.expr);

        if expr_idents.is_empty() {
            return Ok(())
        }

        for block_span in context.blocks.iter().rev() {
            let block_state = fn_state.block_states.get_mut(block_span).unwrap();

            for (_, var_state) in block_state.var_states.iter_mut() {
                if var_state.name == expr_idents[0].as_str() {
                    var_state.used = true;

                    //
                    // TODO: update state of specific fields/tuple members
                    //

                    break;
                }
            }
        }

        Ok(())
    }
}
