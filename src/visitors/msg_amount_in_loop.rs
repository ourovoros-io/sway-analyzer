use super::{AstVisitor, BlockContext, ExprContext, FnContext, WhileExprContext};
use crate::{error::Error, project::Project};
use std::collections::HashMap;
use sway_ast::*;
use sway_types::{Span, Spanned};

#[derive(Default)]
pub struct MsgAmountInLoopVisitor {
    fn_states: HashMap<Span, FnState>,
}

#[derive(Default)]
pub struct FnState {
    block_states: HashMap<Span, BlockState>,
}

#[derive(Default)]
pub struct BlockState {
    is_loop: bool,
    msg_amount_spans: Vec<Span>,
}

impl AstVisitor for MsgAmountInLoopVisitor {
    fn visit_fn(&mut self, context: &FnContext, _project: &mut Project) -> Result<(), crate::error::Error> {
        // Create the function state
        let fn_signature = context.item_fn.fn_signature.span();

        if !self.fn_states.contains_key(&fn_signature) {
            self.fn_states.insert(fn_signature, FnState::default());
        }

        Ok(())
    }

    fn visit_block(&mut self, context: &BlockContext, _project: &mut Project) -> Result<(), crate::error::Error> {
        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = self.fn_states.get_mut(&fn_signature).unwrap();

        // Create the block state
        let block_span = context.block.span();
        
        if !fn_state.block_states.contains_key(&block_span) {
            fn_state.block_states.insert(block_span, BlockState::default());
        }

        Ok(())
    }

    fn leave_block(&mut self, context: &BlockContext, project: &mut Project) -> Result<(), crate::error::Error> {
        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = self.fn_states.get(&fn_signature).unwrap();

        // Get the block state
        let block_span = context.block.span();
        let block_state = fn_state.block_states.get(&block_span).unwrap();

        if block_state.msg_amount_spans.is_empty() {
            return Ok(())
        }

        let mut lines = vec![];

        for msg_amount_span in block_state.msg_amount_spans.iter() {
            let line = project.span_to_line(context.path, msg_amount_span)?.unwrap();
            
            if !lines.contains(&line) {
                lines.push(line);
            }
        }

        let mut blocks = context.blocks.clone();
        blocks.push(block_span);

        for block_span in blocks.iter().rev() {
            let block_state = fn_state.block_states.get(block_span).unwrap();

            if block_state.is_loop {
                for line in lines {
                    project.report.borrow_mut().add_entry(
                        context.path,
                        Some(line),
                        "Found `msg_amount()` call in a loop. Store the value in a variable outside the loop and decrement it over each iteration.".to_string(),
                    );
                }
                break;
            }
        }
        
        Ok(())
    }

    fn visit_while_expr(&mut self, context: &WhileExprContext, _project: &mut Project) -> Result<(), Error> {
        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = self.fn_states.get_mut(&fn_signature).unwrap();

        // Get or create the block state
        let block_span = context.body.span();
        let block_state = fn_state.block_states.entry(block_span).or_insert_with(|| BlockState::default());
        
        // Mark the block as a loop
        block_state.is_loop = true;

        Ok(())
    }

    fn visit_expr(&mut self, context: &ExprContext, _project: &mut Project) -> Result<(), Error> {
        //
        // TODO: this may need to be improved for cases like the following:
        // * `std::context::msg_amount()`
        // * `use std::context; context::msg_amount()`
        // * etc...
        //

        let Expr::FuncApp { func, .. } = context.expr else { return Ok(()) };
        let Expr::Path(path) = func.as_ref() else { return Ok(()) };
        
        if path.prefix.name.as_str() == "msg_amount" {
            // Get the function state
            let fn_signature = context.item_fn.fn_signature.span();
            let fn_state = self.fn_states.get_mut(&fn_signature).unwrap();
    
            // Get the block state
            let block_span = context.blocks.last().unwrap();
            let block_state = fn_state.block_states.get_mut(block_span).unwrap();

            // Add the `msg_amount` span to the block state
            block_state.msg_amount_spans.push(context.expr.span());
        }

        Ok(())
    }
}
