use crate::{
    error::Error,
    project::Project,
    report::Severity,
    utils,
    visitor::{AstVisitor, BlockContext, FnContext, ModuleContext, StatementContext},
};
use std::{collections::HashMap, path::PathBuf};
use sway_ast::Ty;
use sway_types::{BaseIdent, Span, Spanned};

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
    storage_field_types: HashMap<String, Ty>,
}

#[derive(Default)]
struct FnState {
    block_states: HashMap<Span, BlockState>,
}

#[derive(Default)]
struct BlockState {
    storage_value_bindings: Vec<StorageValueBinding>,
}

impl BlockState {
    fn find_last_storage_binding<F>(&mut self, f: F) -> Option<&mut StorageValueBinding>
    where
        F: FnMut(&&mut StorageValueBinding) -> bool,
    {
        self.storage_value_bindings.iter_mut().rev().find(f)
    }
}

struct StorageValueBinding {
    storage_name: BaseIdent,
    variable_name: BaseIdent,
    post_write_name: Option<BaseIdent>,
    shadowing_variable_name: Option<BaseIdent>,
    modified: bool,
    written: bool,
}

impl AstVisitor for StorageNotUpdatedVisitor {
    fn visit_module(&mut self, context: &ModuleContext, _project: &mut Project) -> Result<(), Error> {
        // Get or create the module state
        let module_state = self.module_states.entry(context.path.into()).or_insert_with(ModuleState::default);

        // Store the storage field types ahead of time
        for storage_field in utils::collect_storage_fields(context.module) {
            module_state.storage_field_types.insert(
                storage_field.name.as_str().into(),
                storage_field.ty.clone()
            );
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

        // Check all storage value bindings to see if they are modified or shadowed without being written back to storage
        for storage_value_binding in block_state.storage_value_bindings.iter() {
            if !storage_value_binding.written {
                project.report.borrow_mut().add_entry(
                    context.path,
                    project.span_to_line(context.path, &storage_value_binding.variable_name.span())?,
                    Severity::High,
                    if let Some(shadowing_variable_name) = storage_value_binding.shadowing_variable_name.as_ref() {
                        format!(
                            "The `{}` function has storage bound to local variable `{}` which is shadowed{} before being written back to `storage.{}`.",
                            if let Some(item_impl) = context.item_impl.as_ref() {
                                format!(
                                    "{}::{}",
                                    item_impl.ty.span().as_str(),
                                    context.item_fn.fn_signature.name.as_str(),
                                )
                            } else {
                                format!(
                                    "{}",
                                    context.item_fn.fn_signature.name.as_str(),
                                )
                            },
                            storage_value_binding.variable_name.as_str(),
                            if let Some(line) = project.span_to_line(context.path, &shadowing_variable_name.span())? {
                                format!(" at L{}", line)
                            } else {
                                String::new()
                            },
                            storage_value_binding.storage_name.as_str(),
                        )
                    } else {
                        format!(
                            "The `{}` function has storage bound to local variable `{}` which is not written back to `storage.{}`.",
                            if let Some(item_impl) = context.item_impl.as_ref() {
                                format!(
                                    "{}::{}",
                                    item_impl.ty.span().as_str(),
                                    context.item_fn.fn_signature.name.as_str(),
                                )
                            } else {
                                format!(
                                    "{}",
                                    context.item_fn.fn_signature.name.as_str(),
                                )
                            },
                            storage_value_binding.variable_name.as_str(),
                            storage_value_binding.storage_name.as_str(),
                        )
                    },
                );
            } else if let Some(post_write_name) = storage_value_binding.post_write_name.as_ref() {
                project.report.borrow_mut().add_entry(
                    context.path,
                    project.span_to_line(context.path, &post_write_name.span())?,
                    Severity::High,
                    format!(
                        "The `{}` function has storage bound to local variable `{}` which is updated after writing back to `storage.{}` without writing updated value.",
                        if let Some(item_impl) = context.item_impl.as_ref() {
                            format!(
                                "{}::{}",
                                item_impl.ty.span().as_str(),
                                context.item_fn.fn_signature.name.as_str(),
                            )
                        } else {
                            format!(
                                "{}",
                                context.item_fn.fn_signature.name.as_str(),
                            )
                        },
                        storage_value_binding.variable_name.as_str(),
                        storage_value_binding.storage_name.as_str(),
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
        let Some(block_span) = context.blocks.last() else { return Ok(()) };
        let block_state = fn_state.block_states.get_mut(block_span).unwrap();

        // Check for storage value binding shadowing
        if let Some(variable_name) = utils::statement_to_variable_binding_ident(context.statement) {
            if let Some(storage_value_binding) = block_state.find_last_storage_binding(|x| x.variable_name == variable_name) {
                storage_value_binding.shadowing_variable_name = Some(variable_name.clone());
            }
        }

        //
        // TODO: check for storage key bindings
        //
        // let _ = storage.balances;
        // -> StorageKey<StorageMap<Address, StorageKey<StorageMap<ContractId, u64>>>>
        //
        // let _ = storage.balances.get(accounts.get(i).unwrap());
        // -> StorageKey<StorageMap<ContractId, u64>>
        //

        // Check for storage value binding declaration, i.e: `let mut x = storage.x.read();`
        if let Some((storage_name, variable_name)) = utils::statement_to_storage_read_binding_idents(context.statement) {
            block_state.storage_value_bindings.push(StorageValueBinding {
                storage_name,
                variable_name,
                post_write_name: None,
                shadowing_variable_name: None,
                modified: false,
                written: false,
            });
        }
        // Check for updates to storage value binding, i.e: `x += 1;`
        else if let Some(variable_names) = utils::statement_to_reassignment_idents(context.statement) {
            let variable_name = variable_names.first().unwrap();
            
            for block_span in context.blocks.iter().rev() {
                let block_state = fn_state.block_states.get_mut(block_span).unwrap();

                if let Some(storage_value_binding) = block_state.find_last_storage_binding(|x| x.variable_name == *variable_name) {
                    storage_value_binding.modified = true;

                    // If the storage value binding was previously written, storage is now out of date
                    if storage_value_binding.written {
                        storage_value_binding.post_write_name = Some(variable_name.clone());
                    }

                    break;
                }
            }
        }
        // Check for storage value binding update, i.e: `storage.x.write(x);`
        else if let Some((storage_name, variable_name)) = utils::statement_to_storage_write_idents(context.statement) {
            for block_span in context.blocks.iter().rev() {
                let block_state = fn_state.block_states.get_mut(block_span).unwrap();

                if let Some(storage_value_binding) = block_state.find_last_storage_binding(|x| x.storage_name == storage_name) {
                    if variable_name == storage_value_binding.variable_name {
                        storage_value_binding.written = true;
                        break;
                    }
                }
            }
        }

        Ok(())
    }
}
