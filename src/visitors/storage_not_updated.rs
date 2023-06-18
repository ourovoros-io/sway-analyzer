use super::{AstVisitor, FnContext, StatementContext};
use crate::{error::Error, project::Project, utils};
use std::collections::HashMap;
use sway_ast::{*, attribute::{Attribute, AttributeArg}};
use sway_types::{BaseIdent, Spanned};

//
// If a storage value is bound to a mutable local variable, we need to
// keep track of it in order to assure it gets written back to storage.
//

struct StorageBinding {
    storage_name: BaseIdent,
    variable_name: BaseIdent,
    shadowing_variable_name: Option<BaseIdent>,
    modified: bool,
    written: bool,
}

#[derive(Default)]
struct FnState {
    storage_bindings: Vec<StorageBinding>,
}

#[derive(Default)]
pub struct StorageNotUpdatedVisitor {
    fn_states: HashMap<String, FnState>,
}

impl AstVisitor for StorageNotUpdatedVisitor {
    fn visit_fn(&mut self, context: &FnContext, _project: &mut Project) -> Result<(), Error> {
        if !self.fn_states.contains_key(context.item_fn.fn_signature.name.as_str()) {
            self.fn_states.insert(context.item_fn.fn_signature.name.as_str().to_string(), FnState::default());
        }

        Ok(())
    }

    fn leave_fn(&mut self, context: &FnContext, project: &mut Project) -> Result<(), Error> {
        let fn_state = self.fn_states.get_mut(context.item_fn.fn_signature.name.as_str()).unwrap();

        for storage_binding in fn_state.storage_bindings.iter() {
            if !storage_binding.written {
                project.report.borrow_mut().add_entry(
                    context.path,
                    project.span_to_line(context.path, &storage_binding.variable_name.span())?,
                    if let Some(shadowing_variable_name) = storage_binding.shadowing_variable_name.as_ref() {
                        format!(
                            "Storage bound to local variable `{}` is shadowed{} before being written back to `storage.{}`",
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
                            "Storage bound to local variable `{}` not written back to `storage.{}`",
                            storage_binding.variable_name.as_str(),
                            storage_binding.storage_name.as_str(),
                        )
                    },
                );
            }
        }

        Ok(())
    }

    fn visit_statement(&mut self, context: &StatementContext, _project: &mut Project) -> Result<(), Error> {
        let mut has_storage_write_attribute = false;

        // Check for `#[storage(write)]` attribute
        'attribute_check: for attribute_decl in context.fn_attributes {
            let check_attribute = |attribute: &Attribute| -> bool {
                let check_attribute_arg = |attribute_arg: &AttributeArg| -> bool {
                    attribute_arg.name.as_str() == "write" && attribute_arg.value.is_none()
                };

                if attribute.name.as_str() == "storage" {
                    if let Some(args) = attribute.args.as_ref() {
                        for arg in args.inner.value_separator_pairs.iter() {
                            if check_attribute_arg(&arg.0) {
                                return true;
                            }
                        }

                        if let Some(arg) = args.inner.final_value_opt.as_ref() {
                            if check_attribute_arg(arg) {
                                return true;
                            }
                        }
                    }
                }

                false
            };

            for attribute in attribute_decl.attribute.inner.value_separator_pairs.iter() {
                if check_attribute(&attribute.0) {
                    has_storage_write_attribute = true;
                    break 'attribute_check;
                }
            }

            if let Some(attribute) = attribute_decl.attribute.inner.final_value_opt.as_ref() {
                if check_attribute(attribute) {
                    has_storage_write_attribute = true;
                    break 'attribute_check;
                }
            }
        }

        if !has_storage_write_attribute {
            return Ok(());
        }

        let fn_state = self.fn_states.get_mut(context.item_fn.fn_signature.name.as_str()).unwrap();

        let get_variable_binding_ident = || -> Option<BaseIdent> {
            let Statement::Let(stmt_let) = context.statement else { return None };
            
            let Pattern::Var {
                name: variable_name,
                ..
            } = &stmt_let.pattern else { return None };
            
            Some(variable_name.clone())
        };

        let get_storage_read_binding_idents = || -> Option<(BaseIdent, BaseIdent)> {
            let Statement::Let(stmt_let) = context.statement else { return None };
            
            let Pattern::Var {
                mutable: Some(_),
                name: variable_name,
                ..
            } = &stmt_let.pattern else { return None };
            
            let base_idents = utils::fold_expr_base_idents(&stmt_let.expr);

            if base_idents.len() < 3 {
                return None;
            }

            if base_idents[0].as_str() != "storage" {
                return None;
            }

            if base_idents.last().unwrap().as_str() != "read" {
                return None;
            }

            let storage_name = &base_idents[1];
    
            Some((storage_name.clone(), variable_name.clone()))
        };

        let get_reassignment_ident = || -> Option<BaseIdent> {
            let Statement::Expr {
                expr,
                ..
            } = context.statement else { return None };

            let Expr::Reassignment {
                assignable,
                ..
            } = expr else { return None };
            
            utils::fold_assignable_base_idents(assignable).first().cloned()
        };

        let get_storage_write_idents = || -> Option<(BaseIdent, BaseIdent)> {
            let Statement::Expr {
                expr,
                ..
            } = context.statement else { return None };

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
        };

        // Check for storage binding variable shadowing
        if let Some(variable_name) = get_variable_binding_ident() {
            if let Some(storage_binding) = fn_state.storage_bindings.iter_mut().rev().find(|x| x.variable_name == variable_name) {
                storage_binding.shadowing_variable_name = Some(variable_name.clone());
            }
        }

        // Check for storage binding declaration, i.e: `let mut x = storage.x.read();`
        if let Some((storage_name, variable_name)) = get_storage_read_binding_idents() {
            fn_state.storage_bindings.push(StorageBinding {
                storage_name,
                variable_name,
                shadowing_variable_name: None,
                modified: false,
                written: false,
            });
        }
        // Check for updates to storage binding, i.e: `x += 1;`
        else if let Some(variable_name) = get_reassignment_ident() {
            if let Some(storage_binding) = fn_state.storage_bindings.iter_mut().rev().find(|x| x.variable_name == variable_name) {
                storage_binding.modified = true;
            }
        }
        // Check for storage binding update, i.e: `storage.x.write(x);`
        else if let Some((storage_name, variable_name)) = get_storage_write_idents() {
            if let Some(storage_binding) = fn_state.storage_bindings.iter_mut().rev().find(|x| x.storage_name == storage_name) {
                if variable_name == storage_binding.variable_name {
                    storage_binding.written = true;
                }
            }
        }

        Ok(())
    }
}
