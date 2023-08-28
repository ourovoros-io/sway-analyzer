use crate::{
    error::Error,
    project::Project,
    report::Severity,
    utils,
    visitor::{AstVisitor, BlockContext, ExprContext, FnContext, ModuleContext, StatementContext},
};
use std::{collections::HashMap, path::PathBuf};
use sway_ast::{Expr, Pattern, Statement, StatementLet};
use sway_types::{BaseIdent, Span, Spanned};

#[derive(Default)]
pub struct StorageFieldMutabilityVisitor {
    module_states: HashMap<PathBuf, ModuleState>,
}

#[derive(Default)]
struct ModuleState {
    storage_field_states: HashMap<Span, StorageFieldState>,
    fn_states: HashMap<Span, FnState>,
}

#[derive(Default)]
struct StorageFieldState {
    mutated: bool,
}

#[derive(Default)]
struct FnState {
    block_states: HashMap<Span, BlockState>,
}

#[derive(Default)]
struct BlockState {
    storage_bindings: Vec<StorageBinding>,
}

struct StorageBinding {
    variable_name: BaseIdent,
    storage_name: BaseIdent,
}

impl AstVisitor for StorageFieldMutabilityVisitor {
    fn visit_module(&mut self, context: &ModuleContext, _project: &mut Project) -> Result<(), Error> {
        // Create the module state
        let module_state = self.module_states.entry(context.path.into()).or_insert_with(ModuleState::default);

        // Create storage field states ahead of time
        for storage_field in utils::collect_storage_fields(context.module) {
            module_state.storage_field_states.insert(storage_field.name.span(), StorageFieldState::default());
        }

        Ok(())
    }

    fn leave_module(&mut self, context: &ModuleContext, project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get(context.path).unwrap();

        // Check to see if any storage fields are not mutated
        for (storage_field_span, state) in module_state.storage_field_states.iter() {
            if !state.mutated {
                project.report.borrow_mut().add_entry(
                    context.path,
                    project.span_to_line(context.path, storage_field_span)?,
                    Severity::Low,
                    format!(
                        "The `{}` storage field is never mutated. Consider refactoring it into a constant or a configurable field.",
                        storage_field_span.as_str()
                    ),
                );
            }
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

        // Check for storage binding variable declarations
        let Statement::Let(StatementLet {
            pattern: Pattern::AmbiguousSingleIdent(variable_name),
            expr,
            ..
        }) = context.statement else { return Ok(()) };

        let idents = utils::fold_expr_idents(expr);

        if idents.len() < 3 {
            return Ok(());
        }

        let "storage" = idents[0].as_str() else { return Ok(()) };
        let ("get" | "read") = idents.last().unwrap().as_str() else { return Ok(()) };

        // Create the storage binding
        block_state.storage_bindings.push(StorageBinding {
            variable_name: variable_name.clone(),
            storage_name: idents[1].clone(),
        });

        Ok(())
    }

    fn visit_expr(&mut self, context: &ExprContext, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Get the function state
        let Some(item_fn) = context.item_fn else { return Ok(()) };
        let fn_signature = item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get_mut(&fn_signature).unwrap();

        // Only check method call expressions
        let Expr::MethodCall { .. } = context.expr else { return Ok(()) };

        // Get the expression's identifiers
        let idents = utils::fold_expr_idents(context.expr);

        // Check for `storage.x.write(y)` or `storage.x.insert(y)`
        let ("write" | "insert") = idents.last().unwrap().as_str() else { return Ok(()) };

        // Check the expression for direct storage access
        if idents[0].as_str() == "storage" {
            // Mark the storage field as mutated
            let Some((_, state)) = module_state.storage_field_states.iter_mut().find(|(k, _)| k.as_str() == idents[1].as_str()) else { return Ok(()) };
            state.mutated = true;
        }
        // Check the expression for indirect storage access
        else {
            for block_span in context.blocks.iter().rev() {
                let block_state = fn_state.block_states.get(block_span).unwrap();

                if let Some(storage_binding) = block_state.storage_bindings.iter().rev().find(|x| x.variable_name.as_str() == idents[0].as_str()) {
                    if let Some((_, storage_field_state)) = module_state.storage_field_states.iter_mut().find(|(k, _)| k.as_str() == storage_binding.storage_name.as_str()) {
                        storage_field_state.mutated = true;
                        return Ok(());
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Options;

    #[test]
    fn test_storage_field_mutability() {
        let options = Options {
            directory: Some(PathBuf::from("test/storage_field_mutability")),
            detectors: vec!["storage_field_mutability".to_string()],
            ..Default::default()
        };

        let mut project = Project::try_from(&options).unwrap();
        project.analyze_modules().unwrap();

        println!("{project}");
    }
}
