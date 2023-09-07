use crate::{
    error::Error,
    project::Project,
    report::Severity,
    utils,
    visitor::{AstVisitor, BlockContext, ExprContext, FnContext, ModuleContext, UseContext, StatementLetContext, AstVisitorRecursive},
};
use std::{collections::HashMap, path::PathBuf};
use sway_ast::Expr;
use sway_types::{Span, Spanned};

#[derive(Default)]
pub struct UnprotectedStorageVariablesVisitor {
    module_states: HashMap<PathBuf, ModuleState>,
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

pub struct VarState {
    pub name: String,
    pub is_msg_sender: bool,
}

impl AstVisitor for UnprotectedStorageVariablesVisitor {
    fn visit_module(&mut self, context: &ModuleContext, project: &mut Project) -> Result<(), Error> {
        // Create the module state
        if !self.module_states.contains_key(context.path) {
            self.module_states.insert(context.path.into(), ModuleState::default());
        }

        // Check all functions ahead of time
        let mut fn_state_visitor = AstVisitorRecursive::default();

        fn_state_visitor.visit_fn_hooks.push(Box::new(|context: &FnContext, _project: &mut Project| -> Result<(), Error> {
            // Get the module state
            let module_state = self.module_states.get_mut(context.path).unwrap();
    
            // Get the function state
            let fn_signature = context.item_fn.fn_signature.span();
            let fn_state = module_state.fn_states.entry(fn_signature).or_insert_with(FnState::default);

            // Check if the function has a storage write attribute
            if utils::check_attribute_decls(context.fn_attributes, "storage", &["write"]) {
                fn_state.has_storage_write = true;
            }
    
            Ok(())
        }));

        fn_state_visitor.visit_module(context, project)?;
        fn_state_visitor.leave_module(context, project)?;
        
        std::mem::drop(fn_state_visitor);

        // Get the module state
        let module_state = self.module_states.get(context.path).unwrap();

        for (fn_signature, fn_state) in module_state.fn_states.iter() {
            println!("{}:", fn_signature.as_str());
            println!("\thas storage write? {}", fn_state.has_storage_write);
            println!("\thas msg_sender check? {}", fn_state.has_msg_sender_check);
        }

        Ok(())
    }

    fn visit_use(&mut self, context: &UseContext, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Check the use tree for `std::auth::msg_sender`
        if let Some(name) = utils::use_tree_to_name(&context.item_use.tree, "std::auth::msg_sender") {
            module_state.msg_sender_names.push(name);
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

    fn leave_fn(&mut self, context: &FnContext, project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get(context.path).unwrap();

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

    fn visit_statement_let(&mut self, context: &StatementLetContext, _project: &mut Project) -> Result<(), Error> {
        //
        // TODO: create variable states for each declaration in the let pattern
        //

        let idents = utils::fold_pattern_idents(&context.statement_let.pattern);

        //
        // TODO: check if let value is msg_sender
        //
        // let sender = msg_sender().unwrap();
        //
        // let sender = match msg_sender() {
        //     Ok(sender) => sender,
        //     Err(_) => revert(0),
        // };
        //

        Ok(())
    }

    fn visit_expr(&mut self, context: &ExprContext, _project: &mut Project) -> Result<(), Error> {
        //
        // TODO
        //

        Ok(())
    }

    //
    // TODO: check if any functions write to storage without checking `msg_sender()` via require or if/revert
    //
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_unprotected_storage_variables() {
        crate::tests::test_detector("unprotected_storage_variables")
    }
}
