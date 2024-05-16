use crate::{
    error::Error,
    project::Project,
    report::Severity,
    scope::AstScope,
    utils,
    visitor::{
        AsmInstructionContext, AstVisitor, BlockContext, ExprContext, FnContext, IfExprContext,
        ModuleContext, StatementLetContext, UseContext,
    },
};
use std::{cell::RefCell, collections::HashMap, path::PathBuf, rc::Rc};
use sway_ast::{Expr, IfCondition, Pattern};
use sway_types::{Span, Spanned};

#[derive(Default)]
pub struct ArbitraryCodeExecutionVisitor {
    module_states: HashMap<PathBuf, ModuleState>,
}

struct ModuleState {
    msg_sender_names: Vec<String>,
    fn_states: HashMap<Span, FnState>,
}

impl Default for ModuleState {
    fn default() -> Self {
        Self {
            // Since `std::auth::msg_sender` is part of the prelude, include it here
            msg_sender_names: vec!["msg_sender".into()],
            fn_states: Default::default(),
        }
    }
}

impl ModuleState {
    fn expr_is_msg_sender_call(&mut self, expr: &Expr) -> bool {
        match expr {
            Expr::FuncApp { func, .. } => {
                for name in self.msg_sender_names.iter() {
                    if func.span().as_str() == name || func.span().as_str() == "std::auth::msg_sender" {
                        return true;
                    }
                }
                false
            }

            Expr::MethodCall { target, .. } => self.expr_is_msg_sender_call(target.as_ref()),
            
            Expr::Match { value, .. } => self.expr_is_msg_sender_call(value.as_ref()),

            _ => false,
        }
    }

    fn expr_contains_msg_sender_call(&mut self, expr: &Expr) -> bool {
        match expr {
            Expr::Equal { lhs, rhs, .. } |
            Expr::NotEqual { lhs, rhs, .. } |
            Expr::LogicalAnd { lhs, rhs, .. } |
            Expr::LogicalOr { lhs, rhs, .. } => {
                self.expr_contains_msg_sender_call(lhs.as_ref()) || self.expr_contains_msg_sender_call(rhs.as_ref())
            }

            _ => self.expr_is_msg_sender_call(expr),
        }
    }
}

#[derive(Default)]
struct FnState {
    block_states: HashMap<Span, BlockState>,
}

impl FnState {
    fn expr_is_msg_sender_var(&mut self, expr: &Expr, blocks: &[Span]) -> bool {
        for block_span in blocks.iter().rev() {
            let block_state = self.block_states.get_mut(block_span).unwrap();

            if block_state.expr_is_msg_sender_var(expr) {
                return true;
            }
        }

        false
    }

    fn expr_contains_msg_sender_var(&mut self, expr: &Expr, blocks: &[Span]) -> bool {
        for block_span in blocks.iter().rev() {
            let block_state = self.block_states.get_mut(block_span).unwrap();

            if block_state.expr_contains_msg_sender_var(expr) {
                return true;
            }
        }

        false
    }

    fn has_msg_sender_check(&self, blocks: &[Span]) -> bool {
        for block_span in blocks.iter().rev() {
            let block_state = self.block_states.get(block_span).unwrap();

            if block_state.has_msg_sender_check {
                return true;
            }
        }

        false
    }
}

#[derive(Default)]
struct BlockState {
    var_states: Vec<VarState>,
    has_msg_sender_check: bool,
}

impl BlockState {
    fn expr_is_msg_sender_var(&mut self, expr: &Expr) -> bool {
        match expr {
            Expr::Path(_) => {
                for var_state in self.var_states.iter().rev() {
                    if var_state.name == expr.span().as_str() {
                        return var_state.is_msg_sender;
                    }
                }

                false
            }

            _ => false,
        }
    }

    fn expr_contains_msg_sender_var(&mut self, expr: &Expr) -> bool {
        match expr {
            Expr::Equal { lhs, rhs, .. } |
            Expr::NotEqual { lhs, rhs, .. } |
            Expr::LogicalAnd { lhs, rhs, .. } |
            Expr::LogicalOr { lhs, rhs, .. } => {
                self.expr_contains_msg_sender_var(lhs.as_ref()) || self.expr_contains_msg_sender_var(rhs.as_ref())
            }

            _ => self.expr_is_msg_sender_var(expr),
        }
    }
}

pub struct VarState {
    pub name: String,
    pub is_msg_sender: bool,
}

impl AstVisitor for ArbitraryCodeExecutionVisitor {
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

        // Check the use tree for `std::auth::msg_sender`
        if let Some(name) = utils::use_tree_to_name(&context.item_use.tree, "std::auth::msg_sender") {
            module_state.msg_sender_names.push(name);
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

    fn visit_statement_let(&mut self, context: &StatementLetContext, _scope: Rc<RefCell<AstScope>>, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Check if the variable stores `msg_sender()`
        let mut is_msg_sender = module_state.expr_is_msg_sender_call(&context.statement_let.expr);

        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get_mut(&fn_signature).unwrap();

        // Check if the expression is a variable bound to `msg_sender()`
        if !is_msg_sender && fn_state.expr_is_msg_sender_var(&context.statement_let.expr, context.blocks.as_slice()) {
            is_msg_sender = true;
        }

        // Get the current block state
        let block_span = context.blocks.last().unwrap();
        let block_state = fn_state.block_states.get_mut(block_span).unwrap();
        
        // Add the variable state(s) to the current block state
        match &context.statement_let.pattern {
            Pattern::AmbiguousSingleIdent(ident) => {
                block_state.var_states.push(VarState {
                    name: ident.as_str().to_string(),
                    is_msg_sender,
                });
            }

            pattern => {
                for ident in utils::fold_pattern_idents(pattern) {
                    block_state.var_states.push(VarState {
                        name: ident.as_str().to_string(),
                        is_msg_sender: false,
                    });
                }
            }
        }

        Ok(())
    }

    fn visit_if_expr(&mut self, context: &IfExprContext, _scope: Rc<RefCell<AstScope>>, _project: &mut Project) -> Result<(), Error> {
        // Only check `if let` expressions
        let IfCondition::Let { lhs, rhs, .. } = &context.if_expr.condition else { return Ok(()) };

        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Check if the variable stores `msg_sender()`
        let mut is_msg_sender = module_state.expr_is_msg_sender_call(rhs.as_ref());

        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get_mut(&fn_signature).unwrap();

        // Check if the expression is a variable bound to `msg_sender()`
        if !is_msg_sender && fn_state.expr_is_msg_sender_var(rhs.as_ref(), context.blocks.as_slice()) {
            is_msg_sender = true;
        }
        
        // Get or create the if expression's body block state
        let block_span = context.if_expr.then_block.span();
        let block_state = fn_state.block_states.entry(block_span).or_default();
        
        // Add the variable state(s) to the if expression's body block state
        match lhs.as_ref() {
            Pattern::AmbiguousSingleIdent(ident) => {
                block_state.var_states.push(VarState {
                    name: ident.as_str().to_string(),
                    is_msg_sender,
                });
            }

            pattern => {
                for ident in utils::fold_pattern_idents(pattern) {
                    block_state.var_states.push(VarState {
                        name: ident.as_str().to_string(),
                        is_msg_sender: false,
                    });
                }
            }
        }

        Ok(())
    }

    fn visit_expr(&mut self, context: &ExprContext, _scope: Rc<RefCell<AstScope>>, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Check for a `require` or `if`-`revert`
        let expr = if let Some(require_args) = utils::get_require_args(context.expr) {
            let Some(expr) = require_args.first() else { return Ok(()) };
            expr
        } else if let Some(IfCondition::Expr(expr)) = utils::get_if_revert_condition(context.expr) {
            expr.as_ref()
        } else {
            return Ok(());
        };
        
        let mut has_msg_sender = module_state.expr_contains_msg_sender_call(expr);

        // Get the function state
        let Some(item_fn) = context.item_fn.as_ref() else { return Ok(()) };
        let fn_signature = item_fn.fn_signature.span();            
        let fn_state = module_state.fn_states.get_mut(&fn_signature).unwrap();

        // Check if the expression is a variable bound to `msg_sender()`
        if !has_msg_sender && fn_state.expr_contains_msg_sender_var(expr, context.blocks.as_slice()) {
            has_msg_sender = true;
        }

        // Get the block state
        let block_span = context.blocks.last().unwrap();
        let block_state = fn_state.block_states.get_mut(block_span).unwrap();

        // Note that the function has a `msg_sender()` check
        if has_msg_sender {
            block_state.has_msg_sender_check = true;
        }

        Ok(())
    }

    fn visit_asm_instruction(&mut self, context: &AsmInstructionContext, _scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get(context.path).unwrap();

        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();            
        let fn_state = module_state.fn_states.get(&fn_signature).unwrap();

        // Check if any of the parent blocks have a `msg_sender()` check
        if fn_state.has_msg_sender_check(context.blocks.as_slice()) {
            return Ok(());
        }

        // Only check `LDC` instructions
        let "ldc" = context.instruction.op_code_ident().as_str() else { return Ok(()) };

        project.report.borrow_mut().add_entry(
            context.path,
            project.span_to_line(context.path, &context.instruction.span())?,
            Severity::High,
            format!(
                "{} uses the `LDC` instruction without access restriction: `{}`. Consider checking against `msg_sender()` in order to limit access.",
                utils::get_item_location(context.item, &context.item_impl, &Some(context.item_fn)),
                context.instruction.span().as_str(),
            ),
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_arbitrary_code_execution() {
        crate::tests::test_detector("arbitrary_code_execution", 1);
    }
}
