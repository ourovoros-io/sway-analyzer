use super::{AstVisitor, BlockContext, FnContext, ModuleContext, StatementContext};
use crate::{error::Error, project::Project, utils};
use std::{collections::HashMap, path::PathBuf};
use sway_types::{BaseIdent, Spanned, Span};

//
// If a storage value is bound to a mutable local variable, we need to
// keep track of it in order to assure it gets written back to storage.
//

#[derive(Default)]
pub struct StorageNotUpdatedVisitor {
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
    storage_bindings: Vec<StorageBinding>,
}

impl BlockState {
    fn find_last_storage_binding<F>(&mut self, f: F) -> Option<&mut StorageBinding>
    where
        F: FnMut(&&mut StorageBinding) -> bool,
    {
        self.storage_bindings.iter_mut().rev().find(f)
    }
}

struct StorageBinding {
    storage_name: BaseIdent,
    variable_name: BaseIdent,
    post_write_name: Option<BaseIdent>,
    shadowing_variable_name: Option<BaseIdent>,
    modified: bool,
    written: bool,
}

impl AstVisitor for StorageNotUpdatedVisitor {
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

    fn leave_block(&mut self, context: &BlockContext, project: &mut Project) -> Result<(), Error> {
        // Check for `#[storage(write)]` attribute
        if !utils::check_attribute_decls(context.fn_attributes, "storage", &["write"]) {
            return Ok(());
        }

        // Get the module state
        let module_state = self.module_states.get(context.path).unwrap();
        
        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get(&fn_signature).unwrap();

        // Get the block state
        let block_span = context.block.span();
        let block_state = fn_state.block_states.get(&block_span).unwrap();

        // Check all storage bindings to see if they are modified or shadowed without being written back to storage
        for storage_binding in block_state.storage_bindings.iter() {
            if !storage_binding.written {
                project.report.borrow_mut().add_entry(
                    context.path,
                    project.span_to_line(context.path, &storage_binding.variable_name.span())?,
                    if let Some(shadowing_variable_name) = storage_binding.shadowing_variable_name.as_ref() {
                        format!(
                            "Storage bound to local variable `{}` is shadowed{} before being written back to `storage.{}`.",
                            storage_binding.variable_name.as_str(),
                            if let Some(line) = project.span_to_line(context.path, &shadowing_variable_name.span())? {
                                format!(" at L{}", line)
                            } else {
                                String::new()
                            },
                            storage_binding.storage_name.as_str(),
                        )
                    } else {
                        format!(
                            "Storage bound to local variable `{}` not written back to `storage.{}`.",
                            storage_binding.variable_name.as_str(),
                            storage_binding.storage_name.as_str(),
                        )
                    },
                );
            } else if let Some(post_write_name) = storage_binding.post_write_name.as_ref() {
                project.report.borrow_mut().add_entry(
                    context.path,
                    project.span_to_line(context.path, &post_write_name.span())?,
                    format!(
                        "Storage bound to local variable `{}` updated after writing back to `storage.{}` without writing updated value.",
                        storage_binding.variable_name.as_str(),
                        storage_binding.storage_name.as_str(),
                    ),
                );
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

        // Check for storage binding variable shadowing
        if let Some(variable_name) = utils::statement_to_variable_binding_ident(context.statement) {
            if let Some(storage_binding) = block_state.find_last_storage_binding(|x| x.variable_name == variable_name) {
                storage_binding.shadowing_variable_name = Some(variable_name.clone());
            }
        }

        // Check for storage binding declaration, i.e: `let mut x = storage.x.read();`
        if let Some((storage_name, variable_name)) = utils::statement_to_storage_read_binding_idents(context.statement) {
            block_state.storage_bindings.push(StorageBinding {
                storage_name,
                variable_name,
                post_write_name: None,
                shadowing_variable_name: None,
                modified: false,
                written: false,
            });
        }
        // Check for updates to storage binding, i.e: `x += 1;`
        else if let Some(variable_names) = utils::statement_to_reassignment_idents(context.statement) {
            let variable_name = variable_names.first().unwrap();
            
            for block_span in context.blocks.iter().rev() {
                let block_state = fn_state.block_states.get_mut(block_span).unwrap();

                if let Some(storage_binding) = block_state.find_last_storage_binding(|x| x.variable_name == *variable_name) {
                    storage_binding.modified = true;

                    // If the storage binding was previously written, storage is now out of date
                    if storage_binding.written {
                        storage_binding.post_write_name = Some(variable_name.clone());
                    }

                    break;
                }
            }
        }
        // Check for storage binding update, i.e: `storage.x.write(x);`
        else if let Some((storage_name, variable_name)) = utils::statement_to_storage_write_idents(context.statement) {
            for block_span in context.blocks.iter().rev() {
                let block_state = fn_state.block_states.get_mut(block_span).unwrap();

                if let Some(storage_binding) = block_state.find_last_storage_binding(|x| x.storage_name == storage_name) {
                    if variable_name == storage_binding.variable_name {
                        storage_binding.written = true;
                        break;
                    }
                }
            }
        }

        Ok(())
    }
}
