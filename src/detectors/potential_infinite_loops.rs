use crate::{
    error::Error,
    project::Project,
    report::Severity,
    visitor::{
        AstVisitor, BlockContext, FnContext, ModuleContext, StatementLetContext, WhileExprContext,
    },
};
use std::{collections::HashMap, path::PathBuf};
use sway_ast::{
    literal::{LitBool, LitBoolType},
    Expr, Literal, PathExpr, Pattern, Statement, StatementLet,
};
use sway_types::{Span, Spanned};

#[derive(Default)]
pub struct PotentialInfiniteLoopsVisitor {
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
    variables: Vec<(String, LitBoolType)>,
}

impl AstVisitor for PotentialInfiniteLoopsVisitor {
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

    fn visit_statement_let(&mut self, context: &StatementLetContext, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get_mut(&fn_signature).unwrap();

        // Get the block state
        let block_span = context.blocks.last().unwrap();
        let block_state = fn_state.block_states.get_mut(block_span).unwrap();

        let StatementLet { expr, pattern, .. } = &context.statement_let;
        let Pattern::AmbiguousSingleIdent(ident) = &pattern else {return Ok(())};
        let Expr::Literal(Literal::Bool(LitBool { kind, .. })) = expr else {return Ok(())};

        block_state.variables.push((ident.span().str(), kind.clone()));

        Ok(())
    }

    fn visit_while_expr(&mut self, context: &WhileExprContext, project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get(context.path).unwrap();
        
        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get(&fn_signature).unwrap();

        let mut start_kind = LitBoolType::True;
        
        match context.condition {
            Expr::Literal(Literal::Bool(LitBool { kind, .. })) => {
                start_kind = kind.clone();
            }

            Expr::Not { expr, .. } => {
                if let Expr::Path(PathExpr { prefix, .. }) = expr.as_ref() {
                    let mut var_kind = None;

                    'var_lookup: for block_span in context.blocks.iter().rev() {
                        let block_state = fn_state.block_states.get(block_span).unwrap();

                        for (name, kind) in block_state.variables.iter().rev() {
                            if name== prefix.span().as_str() {
                                var_kind = Some(kind);
                                break 'var_lookup;
                            }
                        }
                    }

                    let Some(var_kind) = var_kind else { return Ok(()) };

                    match var_kind {
                        LitBoolType::True => start_kind = LitBoolType::False,
                        LitBoolType::False => start_kind = LitBoolType::True,
                    }
                }
            }

            _ => {
                let Expr::Path(PathExpr { prefix, .. }) = context.condition else { return Ok(()) };

                let mut var_kind = None;

                'var_lookup: for block_span in context.blocks.iter().rev() {
                    let block_state = fn_state.block_states.get(block_span).unwrap();

                    for (name, kind) in block_state.variables.iter().rev() {
                        if name== prefix.span().as_str() {
                            var_kind = Some(kind);
                            break 'var_lookup;
                        }
                    }
                }

                if var_kind.is_none() {
                    return Ok(());
                }
            }
        }

        let found_break = context.body.inner.statements.iter().any(|statement| {
            let Statement::Expr { expr, .. } = statement else { return false };
            match expr {
                Expr::Break { .. } => true,

                Expr::Reassignment { expr, .. } => {
                    if let Expr::Literal(Literal::Bool(LitBool { kind, .. })) = expr.as_ref() {
                        *kind == start_kind
                    } else {
                        false
                    }
                }

                _ => false,
            }
        });

        if !found_break {
            project.report.borrow_mut().add_entry(
                context.path,
                project.span_to_line(context.path, &context.expr.span())?,
                Severity::High,
                format!(
                    "The `{}` function contains a potentially infinite loop. Consider adding a `break` statement.",
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
                    }
                ),
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Options;

    #[test]
    fn test_potential_infinite_loop() {
        let options = Options {
            directory: Some(PathBuf::from("test/potential_infinite_loops")),
            detectors: vec!["potential_infinite_loops".to_string()],
            ..Default::default()
        };

        let mut project = Project::try_from(&options).unwrap();
        project.analyze_modules().unwrap();

        println!("{project}");
    }
}
