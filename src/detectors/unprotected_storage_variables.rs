use crate::{
    error::Error,
    project::Project,
    report::Severity,
    utils,
    visitor::{
        AstVisitor, AstVisitorRecursive, ExprContext, FnContext, ModuleContext,
        StatementLetContext, UseContext,
    },
};
use std::{cell::RefCell, collections::HashMap, path::PathBuf, rc::Rc};
use sway_ast::{Expr, IfCondition, IfExpr, ItemImplItem, ItemKind, Pattern};
use sway_types::{Span, Spanned};

#[derive(Default)]
pub struct UnprotectedStorageVariablesVisitor {
    module_states: Rc<RefCell<HashMap<PathBuf, ModuleState>>>,
}

pub struct ModuleState {
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
pub struct FnState {
    block_states: HashMap<Span, BlockState>,
    has_storage_write: bool,
    has_msg_sender_check: bool,
}

#[derive(Default)]
pub struct BlockState {
    var_states: Vec<VarState>,
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

impl AstVisitor for UnprotectedStorageVariablesVisitor {
    fn visit_module(&mut self, context: &ModuleContext, _project: &mut Project) -> Result<(), Error> {
        // Create the module state
        if !self.module_states.borrow().contains_key(context.path) {
            self.module_states.borrow_mut().insert(context.path.into(), ModuleState::default());
        }

        Ok(())
    }

    fn leave_module(&mut self, context: &ModuleContext, project: &mut Project) -> Result<(), Error> {
        let mut postprocess_visitor = AstVisitorRecursive::default();

        // Propogate function states for called functions to the function calling them
        postprocess_visitor.visit_expr_hooks.push(Box::new(|context, _project| {
            // Only check function calls
            let Expr::FuncApp { func, .. } = context.expr else { return Ok(()) };
    
            let mut fn_signature = None;
    
            // Check if function is in toplevel scope
            for item in context.module.items.iter() {
                let ItemKind::Fn(item_fn) = &item.value else { continue };
                
                if item_fn.fn_signature.name.as_str() == func.span().as_str() {
                    fn_signature = Some(item_fn.fn_signature.span());
                    break;
                }
            }
    
            // Check if function is in impl scope
            if fn_signature.is_none() {
                if let Some(item_impl) = context.item_impl.as_ref() {
                    for item in item_impl.contents.inner.iter() {
                        let ItemImplItem::Fn(item_fn) = &item.value else { continue };
                        
                        if item_fn.fn_signature.name.as_str() == func.span().as_str() {
                            fn_signature = Some(item_fn.fn_signature.span());
                            break;
                        }
                    }
                }
            }
    
            // Get the module state
            let mut module_states = self.module_states.borrow_mut();
            let module_state = module_states.get_mut(context.path).unwrap();
    
            // Get the called function state
            let Some(fn_signature) = fn_signature else { return Ok(()) };
            let fn_state = module_state.fn_states.get(&fn_signature).unwrap();
            let has_storage_write = fn_state.has_storage_write;
            let has_msg_sender_check = fn_state.has_msg_sender_check;
            
            // Update the current function state
            let Some(item_fn) = context.item_fn.as_ref() else { return Ok(()) };
            let fn_signature = item_fn.fn_signature.span();
            let fn_state = module_state.fn_states.get_mut(&fn_signature).unwrap();
            
            if has_storage_write {
                fn_state.has_storage_write = true;
            }
    
            if has_msg_sender_check {
                fn_state.has_msg_sender_check = true;
            }
    
            Ok(())
        }));

        // Check functions for missing access restriction
        postprocess_visitor.leave_fn_hooks.push(Box::new(|context, project| {
            // Get the module state
            let module_states = self.module_states.borrow();
            let module_state = module_states.get(context.path).unwrap();
    
            // Get the function state
            let fn_signature = context.item_fn.fn_signature.span();
            let fn_state = module_state.fn_states.get(&fn_signature).unwrap();
    
            if fn_state.has_storage_write && !fn_state.has_msg_sender_check {
                project.report.borrow_mut().add_entry(
                    context.path,
                    project.span_to_line(context.path, &fn_signature)?,
                    Severity::High,
                    format!(
                        "{} writes to storage without access restriction. Consider checking against `msg_sender()` in order to limit access.",
                        utils::get_item_location(context.item, &context.item_impl, &Some(context.item_fn)),
                    ),
                );
            }
    
            Ok(())
        }));

        // Perform postprocessing steps
        postprocess_visitor.visit_module(context, project)?;
        postprocess_visitor.leave_module(context, project)?;

        Ok(())
    }

    fn visit_use(&mut self, context: &UseContext, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let mut module_states = self.module_states.borrow_mut();
        let module_state = module_states.get_mut(context.path).unwrap();

        // Check the use tree for `std::auth::msg_sender`
        if let Some(name) = utils::use_tree_to_name(&context.item_use.tree, "std::auth::msg_sender") {
            module_state.msg_sender_names.push(name);
        }

        Ok(())
    }

    fn visit_fn(&mut self, context: &FnContext, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let mut module_states = self.module_states.borrow_mut();
        let module_state = module_states.get_mut(context.path).unwrap();

        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.entry(fn_signature).or_insert_with(FnState::default);

        // Check if the function has a storage write attribute
        if utils::check_attribute_decls(context.fn_attributes, "storage", &["write"]) {
            fn_state.has_storage_write = true;
        }

        Ok(())
    }

    fn visit_statement_let(&mut self, context: &StatementLetContext, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let mut module_states = self.module_states.borrow_mut();
        let module_state = module_states.get_mut(context.path).unwrap();

        // Check if the variable stores `msg_sender()`
        let mut is_msg_sender = module_state.expr_is_msg_sender_call(&context.statement_let.expr);

        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.entry(fn_signature).or_insert_with(FnState::default);

        let Pattern::AmbiguousSingleIdent(ident) = &context.statement_let.pattern else { return Ok(()) };

        // Check if the variable stores another variable that stores `msg_sender()`
        if !is_msg_sender {
            for block_span in context.blocks.iter().rev() {
                let block_state = fn_state.block_states.entry(block_span.clone()).or_insert_with(BlockState::default);

                if block_state.expr_is_msg_sender_var(&context.statement_let.expr) {
                    is_msg_sender = true;
                    break;
                }
            }
        }

        // Add the variable state to the current block state
        if is_msg_sender {
            let block_span = context.blocks.last().unwrap();
            let block_state = fn_state.block_states.entry(block_span.clone()).or_insert_with(BlockState::default);
            
            block_state.var_states.push(VarState {
                name: ident.as_str().to_string(),
                is_msg_sender,
            });
        }

        Ok(())
    }

    fn visit_expr(&mut self, context: &ExprContext, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let mut module_states = self.module_states.borrow_mut();
        let module_state = module_states.get_mut(context.path).unwrap();

        // Check for require on `msg_sender()`
        if let Some(require_args) = utils::get_require_args(context.expr) {
            for expr in require_args {
                let mut has_msg_sender = module_state.expr_contains_msg_sender_call(expr);

                // Get the function state
                let Some(item_fn) = context.item_fn.as_ref() else { return Ok(()) };
                let fn_signature = item_fn.fn_signature.span();            
                let fn_state = module_state.fn_states.entry(fn_signature).or_insert_with(FnState::default);

                if !has_msg_sender {
                    // Check for `msg_sender` variables in all available blocks
                    for block_span in context.blocks.iter().rev() {
                        let block_state = fn_state.block_states.entry(block_span.clone()).or_insert_with(BlockState::default);

                        if block_state.expr_contains_msg_sender_var(expr) {
                            has_msg_sender = true;
                            break;
                        }
                    }
                }

                if has_msg_sender {
                    // Note that the function has a `msg_sender()` check
                    fn_state.has_msg_sender_check = true;
                    break;
                }
            }
        }
        // Check for if/revert on `msg_sender()`
        else if let Expr::If(IfExpr { condition: IfCondition::Expr(expr), then_block, .. }) = context.expr {
            if !utils::block_has_revert(then_block) {
                return Ok(());
            }

            let mut has_msg_sender = module_state.expr_contains_msg_sender_call(expr.as_ref());

            // Get the function state
            let Some(item_fn) = context.item_fn.as_ref() else { return Ok(()) };
            let fn_signature = item_fn.fn_signature.span();            
            let fn_state = module_state.fn_states.entry(fn_signature).or_insert_with(FnState::default);

            if !has_msg_sender {
                // Check for `msg_sender` variables in all available blocks
                for block_span in context.blocks.iter().rev() {
                    let block_state = fn_state.block_states.entry(block_span.clone()).or_insert_with(BlockState::default);

                    if block_state.expr_contains_msg_sender_var(expr) {
                        has_msg_sender = true;
                        break;
                    }
                }
            }

            if has_msg_sender {
                // Note that the function has a `msg_sender()` check
                fn_state.has_msg_sender_check = true;
                return Ok(());
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_unprotected_storage_variables() {
        crate::tests::test_detector("unprotected_storage_variables")
    }
}
