use crate::{
    error::Error,
    project::Project,
    report::Severity,
    utils,
    visitor::{AstVisitor, BlockContext, ExprContext, FnContext, ModuleContext, StatementContext, WhileExprContext},
};
use std::{collections::HashMap, path::PathBuf};
use sway_ast::{Expr, Statement, StatementLet};
use sway_types::{Span, Spanned};

#[derive(Default)]
pub struct ExternalCallsInLoopVisitor {
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
    is_loop: bool,
    variables: Vec<(Span, bool)>, // (span, is_abi)
}

impl AstVisitor for ExternalCallsInLoopVisitor {
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

    fn visit_while_expr(&mut self, context: &WhileExprContext, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get_mut(&fn_signature).unwrap();

        // Get or create the while expression's body block state
        let block_span = context.body.span();
        let block_state = fn_state.block_states.entry(block_span).or_insert_with(BlockState::default);

        // Mark the while expression's body block as a loop
        block_state.is_loop = true;

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

        // Store variable bindings in the block state
        if let Statement::Let(StatementLet { pattern, expr, .. }) = context.statement {
            let idents = utils::fold_pattern_idents(pattern);

            //
            // TODO: check if `expr` is either a tuple or a function call returning a tuple with the same length as `idents`
            //

            if idents.len() == 1 {
                block_state.variables.push((idents[0].span(), matches!(expr, Expr::AbiCast { .. })));
            } else {
                for ident in idents {
                    block_state.variables.push((ident.span(), false));
                }
            }
        }

        Ok(())
    }

    fn visit_expr(&mut self, context: &ExprContext, project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Get the function state
        let Some(item_fn) = context.item_fn.as_ref() else { return Ok(()) };
        let fn_signature = item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get_mut(&fn_signature).unwrap();

        // Check to see if we are in a loop
        if !context.blocks.iter().rev().any(|block_span| fn_state.block_states.get(&block_span).unwrap().is_loop) {
            return Ok(());
        }

        // Check to see if the expression is a method call
        let Expr::MethodCall { target, .. } = context.expr else { return Ok(()) };

        let add_report_entry = || -> Result<(), Error> {
            project.report.borrow_mut().add_entry(
                context.path,
                project.span_to_line(context.path, &context.expr.span())?,
                Severity::Medium,
                format!(
                    "The `{}` function performs an external call in a loop: `{}`",
                    if let Some(item_impl) = context.item_impl.as_ref() {
                        format!(
                            "{}::{}",
                            item_impl.ty.span().as_str(),
                            item_fn.fn_signature.name.as_str(),
                        )
                    } else {
                        format!(
                            "{}",
                            item_fn.fn_signature.name.as_str(),
                        )
                    },
                    context.expr.span().as_str(),
                ),
            );

            Ok(())
        };

        match target.as_ref() {
            Expr::Path(_) => {
                let target_idents = utils::fold_expr_idents(target.as_ref());

                if target_idents.len() != 1 {
                    return Ok(());
                }

                let target_span = target_idents[0].span();
                
                // Check to see if the method call's target is an abi variable
                for block_span in context.blocks.iter().rev() {
                    let block_state = fn_state.block_states.get_mut(&block_span).unwrap();
                    
                    for (variable_span, is_abi) in block_state.variables.iter().rev() {
                        if variable_span.as_str() == target_span.as_str() && *is_abi {
                            return add_report_entry();
                        }
                    }
                }
            }
            
            Expr::AbiCast { .. } => {
                add_report_entry()?;
            }
            
            _ => {}
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Options;

    #[test]
    fn test_redundant_storage_access() {
        let options = Options {
            directory: Some(PathBuf::from("test/external_calls_in_loop")),
            detectors: vec!["external_calls_in_loop".to_string()],
            ..Default::default()
        };

        let mut project = Project::try_from(&options).unwrap();
        project.analyze_modules().unwrap();

        println!("{project}");
    }
}
