use super::{AstVisitor, BlockContext, FnContext, ModuleContext, StatementContext};
use crate::{error::Error, project::Project, utils};
use std::{collections::HashMap, path::PathBuf};
use sway_types::{Span, Spanned};

#[derive(Default)]
pub struct MissingLogsVisitor {
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
    written: Vec<Span>,
    logged: Vec<Span>,
}

//
// TODO:
//   keep track of storage writes in blocks
//   when leaving a block, check to make sure the storage written is logged
//

impl AstVisitor for MissingLogsVisitor {
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

        // Check for storage writes and add them to the block state
        if let Some((storage_name, _)) = utils::statement_to_storage_write_idents(context.statement) {
            block_state.written.push(storage_name.span());
        }

        //
        // TODO: Check for calls to `std::logging::log`
        //

        Ok(())
    }
}
