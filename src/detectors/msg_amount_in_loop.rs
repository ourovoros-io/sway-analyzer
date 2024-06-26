use crate::{
    error::Error,
    project::Project,
    report::Severity,
    scope::AstScope,
    utils,
    visitor::{
        AstVisitor, BlockContext, ExprContext, FnContext, ModuleContext, UseContext,
        WhileExprContext,
    },
};
use std::{cell::RefCell, collections::HashMap, path::PathBuf, rc::Rc};
use sway_ast::Expr;
use sway_types::{Span, Spanned};

#[derive(Default)]
pub struct MsgAmountInLoopVisitor {
    module_states: HashMap<PathBuf, ModuleState>,
}

#[derive(Default)]
struct ModuleState {
    msg_amount_names: Vec<String>,
    fn_states: HashMap<Span, FnState>,
}

#[derive(Default)]
struct FnState {
    block_states: HashMap<Span, BlockState>,
}

#[derive(Default)]
struct BlockState {
    is_loop: bool,
    msg_amount_spans: Vec<Span>,
}

impl AstVisitor for MsgAmountInLoopVisitor {
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

        // Check the use tree for `std::context::msg_amount`
        if let Some(name) = utils::use_tree_to_name(&context.item_use.tree, "std::context::msg_amount") {
            module_state.msg_amount_names.push(name);
        }
        
        // Check the use tree for `std::registers::balance`
        if let Some(name) = utils::use_tree_to_name(&context.item_use.tree, "std::registers::balance") {
            module_state.msg_amount_names.push(name);
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

    fn visit_block(&mut self, context: &BlockContext, _scope: Rc<RefCell<AstScope>>, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get_mut(&fn_signature).unwrap();

        // Create the block state
        let block_span = context.block.span();
        
        fn_state.block_states.entry(block_span).or_default();

        Ok(())
    }

    fn leave_block(&mut self, context: &BlockContext, _scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get(context.path).unwrap();

        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get(&fn_signature).unwrap();

        // Get the block state
        let block_span = context.block.span();
        let block_state = fn_state.block_states.get(&block_span).unwrap();

        if block_state.msg_amount_spans.is_empty() {
            return Ok(())
        }

        let msg_amount_spans = block_state.msg_amount_spans.clone();

        let mut blocks = context.blocks.clone();
        blocks.push(block_span);

        for block_span in blocks.iter().rev() {
            let block_state = fn_state.block_states.get(block_span).unwrap();

            if block_state.is_loop {
                for msg_amount_span in msg_amount_spans.iter() {
                    project.report.borrow_mut().add_entry(
                        context.path,
                        project.span_to_line(context.path, msg_amount_span)?,
                        Severity::Medium,
                        format!(
                            "{} makes a call to `{}` in a loop. Store the value in a variable outside the loop and decrement it over each iteration.",
                            utils::get_item_location(context.item, &context.item_impl, &Some(context.item_fn)),
                            msg_amount_span.as_str(),
                        ),
                    );
                }
                break;
            }
        }
        
        Ok(())
    }

    fn visit_while_expr(&mut self, context: &WhileExprContext, _scope: Rc<RefCell<AstScope>>, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get_mut(&fn_signature).unwrap();

        // Get or create the block state
        let block_span = context.body.span();
        let block_state = fn_state.block_states.entry(block_span).or_default();
        
        // Mark the block as a loop
        block_state.is_loop = true;

        Ok(())
    }

    fn visit_expr(&mut self, context: &ExprContext, _scope: Rc<RefCell<AstScope>>, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Get the function state
        let Some(item_fn) = context.item_fn.as_ref() else { return Ok(()) };
        let fn_signature = item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get_mut(&fn_signature).unwrap();

        // Get the block state
        let Some(block_span) = context.blocks.last() else { return Ok(()) };
        let block_state = fn_state.block_states.get_mut(block_span).unwrap();

        // Destructure the expression into a function application
        let Expr::FuncApp { func, .. } = context.expr else { return Ok(()) };
        let Expr::Path(path) = func.as_ref() else { return Ok(()) };
        
        // Check for calls to imported `msg_amount` or `balance` functions
        if path.suffix.is_empty() {
            for msg_amount_name in module_state.msg_amount_names.iter() {
                if path.prefix.name.as_str() == msg_amount_name {
                    // Add the `msg_amount` span to the block state
                    block_state.msg_amount_spans.push(context.expr.span());
                    break;
                }
            }
        }
        // Check for calls to `std::context::msg_amount` or `std::registers::balance` functions
        else if path.suffix.len() == 2 {
            let "std" = path.prefix.name.as_str() else { return Ok(()) };
            let ("context" | "registers") = path.suffix[0].1.name.as_str() else { return Ok(()) };
            let ("balance" | "msg_amount") = path.suffix[1].1.name.as_str() else { return Ok(()) };
            block_state.msg_amount_spans.push(context.expr.span());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_msg_amount_in_loop() {
        crate::tests::test_detector("msg_amount_in_loop", 4);
    }
}
