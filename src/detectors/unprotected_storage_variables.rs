use crate::{
    error::Error,
    project::Project,
    utils,
    visitor::{AstVisitor, BlockContext, FnContext, ModuleContext, UseContext},
};
use std::{collections::HashMap, path::PathBuf};
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
    fn visit_module(&mut self, context: &ModuleContext, _project: &mut Project) -> Result<(), Error> {
        // Create the module state
        if !self.module_states.contains_key(context.path) {
            self.module_states.insert(context.path.into(), ModuleState::default());
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
