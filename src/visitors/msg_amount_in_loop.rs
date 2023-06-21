use super::AstVisitor;
use std::collections::HashMap;
use sway_ast::*;
use sway_types::{Span, Spanned};

#[derive(Default)]
pub struct FnState {
    msg_amount_spans: Vec<Span>,
}

#[derive(Default)]
pub struct MsgAmountInLoopVisitor {
    fn_states: HashMap<Span, FnState>,
}

impl AstVisitor for MsgAmountInLoopVisitor {
    fn visit_fn(&mut self, context: &super::FnContext, project: &mut crate::project::Project) -> Result<(), crate::error::Error> {
        // Create the function state
        let fn_signature = context.item_fn.fn_signature.span();

        if !self.fn_states.contains_key(&fn_signature) {
            self.fn_states.insert(fn_signature, FnState::default());
        }

        Ok(())
    }

    fn leave_fn(&mut self, context: &super::FnContext, project: &mut crate::project::Project) -> Result<(), crate::error::Error> {
        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = self.fn_states.get(&fn_signature).unwrap();

        // TODO: check fn_state.msg_amount_spans and create report entries

        Ok(())
    }

    fn visit_expr(&mut self, context: &super::ExprContext, project: &mut crate::project::Project) -> Result<(), crate::error::Error> {
        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let mut fn_state = self.fn_states.get_mut(&fn_signature).unwrap();

        let Expr::While { block, .. } = context.expr else { return Ok(()) };

        Ok(())
    }
}
