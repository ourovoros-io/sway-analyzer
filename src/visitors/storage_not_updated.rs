use super::{AstVisitor, FnContext, StatementContext};
use crate::{error::Error, project::Project, utils};
use std::collections::HashMap;
use sway_ast::*;
use sway_types::{BaseIdent, Spanned, Span};

//
// If a storage value is bound to a mutable local variable, we need to
// keep track of it in order to assure it gets written back to storage.
//

#[derive(Default)]
pub struct StorageNotUpdatedVisitor {
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
    fn visit_fn(&mut self, context: &FnContext, _project: &mut Project) -> Result<(), Error> {
        // Create the function state
        let fn_signature = context.item_fn.fn_signature.span();

        if !self.fn_states.contains_key(&fn_signature) {
            self.fn_states.insert(fn_signature, FnState::default());
        }

        Ok(())
    }

    fn visit_block(&mut self, context: &super::BlockContext, _project: &mut Project) -> Result<(), Error> {
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

    fn leave_block(&mut self, context: &super::BlockContext, project: &mut Project) -> Result<(), Error> {
        // Check for `#[storage(write)]` attribute
        if !utils::check_attribute_decls(context.fn_attributes, "storage", &["write"]) {
            return Ok(());
        }

        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = self.fn_states.get(&fn_signature).unwrap();

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
        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = self.fn_states.get_mut(&fn_signature).unwrap();

        // Get the block state
        let block_span = context.blocks.last().unwrap();
        let block_state = fn_state.block_states.get_mut(&block_span).unwrap();

        // Check for storage binding variable shadowing
        if let Some(variable_name) = get_variable_binding_ident(context.statement) {
            if let Some(storage_binding) = block_state.find_last_storage_binding(|x| x.variable_name == variable_name) {
                storage_binding.shadowing_variable_name = Some(variable_name.clone());
            }
        }

        // Check for storage binding declaration, i.e: `let mut x = storage.x.read();`
        if let Some((storage_name, variable_name)) = get_storage_read_binding_idents(context.statement) {
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
        else if let Some(variable_name) = get_reassignment_ident(context.statement) {
            for block_span in context.blocks.iter().rev() {
                let block_state = fn_state.block_states.get_mut(block_span).unwrap();

                if let Some(storage_binding) = block_state.find_last_storage_binding(|x| x.variable_name == variable_name) {
                    storage_binding.modified = true;

                    // If the storage binding was previously written, storage is now out of date
                    if storage_binding.written {
                        storage_binding.post_write_name = Some(variable_name);
                    }

                    break;
                }
            }
        }
        // Check for storage binding update, i.e: `storage.x.write(x);`
        else if let Some((storage_name, variable_name)) = get_storage_write_idents(context.statement) {
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

fn get_variable_binding_ident(statement: &Statement) -> Option<BaseIdent> {
    let Statement::Let(StatementLet {
        pattern,
        ..
    }) = statement else { return None };
    
    let Pattern::Var {
        name: variable_name,
        ..
    } = pattern else { return None };
    
    Some(variable_name.clone())
}

fn get_storage_read_binding_idents(statement: &Statement) -> Option<(BaseIdent, BaseIdent)> {
    let Statement::Let(StatementLet {
        pattern,
        expr,
        ..
    }) = statement else { return None };
    
    let Pattern::Var {
        mutable: Some(_),
        name: variable_name,
        ..
    } = pattern else { return None };
    
    let storage_idents = utils::fold_expr_base_idents(expr);

    if storage_idents.len() < 3 {
        return None;
    }

    if storage_idents[0].as_str() != "storage" {
        return None;
    }

    if storage_idents.last().unwrap().as_str() != "read" {
        return None;
    }

    let storage_name = &storage_idents[1];

    Some((storage_name.clone(), variable_name.clone()))
}

fn get_reassignment_ident(statement: &Statement) -> Option<BaseIdent> {
    let Statement::Expr {
        expr,
        ..
    } = statement else { return None };

    let Expr::Reassignment {
        assignable,
        ..
    } = expr else { return None };
    
    utils::fold_assignable_base_idents(assignable).first().cloned()
}

fn get_storage_write_idents(statement: &Statement) -> Option<(BaseIdent, BaseIdent)> {
    let Statement::Expr {
        expr,
        ..
    } = statement else { return None };

    let Expr::MethodCall {
        args,
        ..
    } = expr else { return None };

    let storage_idents = utils::fold_expr_base_idents(expr);

    if storage_idents.len() < 3 {
        return None;
    }

    if storage_idents[0].as_str() != "storage" {
        return None;
    }

    let ("write" | "insert") = storage_idents.last().unwrap().as_str() else { return None };

    let variable_idents = utils::fold_expr_base_idents(args.inner.final_value_opt.as_ref().unwrap());

    // TODO: need to support paths with multiple idents
    if variable_idents.len() != 1 {
        return None;
    }

    Some((storage_idents[1].clone(), variable_idents[0].clone()))
}
