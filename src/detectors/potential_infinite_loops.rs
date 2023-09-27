use crate::{
    error::Error,
    project::Project,
    report::Severity,
    utils,
    visitor::{AstVisitor, BlockContext, ExprContext, FnContext, ModuleContext, WhileExprContext},
};
use std::{collections::HashMap, path::PathBuf};
use sway_ast::{Expr, expr::ReassignmentOpVariant, Literal, literal::{LitBool, LitBoolType}};
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
    is_while_loop: bool,
    has_break: bool,
    condition: Option<Expr>,
    condition_updated: bool,
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

    fn leave_block(&mut self, context: &BlockContext, project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get(context.path).unwrap();

        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get(&fn_signature).unwrap();

        // Get the block state
        let block_span = context.block.span();
        let block_state = fn_state.block_states.get(&block_span).unwrap();

        if block_state.is_while_loop && (!block_state.has_break && !block_state.condition_updated) {
            project.report.borrow_mut().add_entry(
                context.path,
                project.span_to_line(context.path, &block_span)?,
                Severity::High,
                format!(
                    "{} contains a potentially infinite loop: `while {} {{ ... }}`. Consider adding a `break` statement.",
                    utils::get_item_location(context.item, &context.item_impl, &Some(context.item_fn)),
                    block_state.condition.as_ref().unwrap().span().as_str(),
                ),
            );
        }

        Ok(())
    }

    fn visit_while_expr(&mut self, context: &WhileExprContext, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();
        
        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get_mut(&fn_signature).unwrap();

        // Create the while expression's body block ahead of time
        let block_span = context.body.span();
        let block_state = fn_state.block_states.entry(block_span).or_insert_with(BlockState::default);

        // Mark the block as a loop and store its condition
        block_state.is_while_loop = true;
        block_state.condition = Some(context.condition.clone());

        Ok(())
    }

    fn visit_expr(&mut self, context: &ExprContext, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();
        
        // Get the function state
        let Some(item_fn) = context.item_fn.as_ref() else { return Ok(()) };
        let fn_signature = item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get_mut(&fn_signature).unwrap();

        match context.expr {
            Expr::Break { .. } => {
                for block_span in context.blocks.iter().rev() {
                    let block_state = fn_state.block_states.get_mut(block_span).unwrap();

                    if block_state.is_while_loop {
                        block_state.has_break = true;
                        break;
                    }
                }
            }

            Expr::Reassignment { assignable, reassignment_op, expr: value } => {
                let mut loop_block_span = None;

                for block_span in context.blocks.iter().rev() {
                    let block_state = fn_state.block_states.get_mut(block_span).unwrap();

                    if block_state.is_while_loop {
                        loop_block_span = Some(block_span);
                        break;
                    }
                }

                let Some(loop_block_span) = loop_block_span else { return Ok(()) };
                let loop_block_state = fn_state.block_states.get_mut(loop_block_span).unwrap();

                if loop_block_state.condition_updated {
                    return Ok(());
                }

                match loop_block_state.condition.as_ref().unwrap() {
                    Expr::FuncApp { .. } | Expr::MethodCall { .. } => {
                        // Mark function conditions as updated by default
                        loop_block_state.condition_updated = true;
                    }

                    expr if matches!(expr, Expr::Path(_)) => {
                        // Check if `expr` is set to `false`
                        let ReassignmentOpVariant::Equals = &reassignment_op.variant else { return Ok(()) };
                        let Expr::Literal(Literal::Bool(LitBool { kind: LitBoolType::False, .. })) = value.as_ref() else { return Ok(()) };
                        let assignable_idents = utils::fold_assignable_idents(assignable);
                        let condition_idents = utils::fold_expr_idents(expr);
                        if assignable_idents.iter().zip(condition_idents).any(|(a, b)| a.as_str() != b.as_str()) {
                            return Ok(());
                        }
                        loop_block_state.condition_updated = true;
                    }

                    Expr::Not { expr, .. } => {
                        // Don't check function applications or method calls
                        if matches!(expr.as_ref(), Expr::FuncApp { .. } | Expr::MethodCall { .. }) {
                            loop_block_state.condition_updated = true;
                            return Ok(());
                        }
                        // Check if `expr` is set to `false` or `true` (depending on the final negation logic)
                        let ReassignmentOpVariant::Equals = &reassignment_op.variant else { return Ok(()) };
                        if utils::expr_negation_result(expr) {
                            let Expr::Literal(Literal::Bool(LitBool { kind: LitBoolType::True, .. })) = value.as_ref() else { return Ok(()) };
                        } else {
                            let Expr::Literal(Literal::Bool(LitBool { kind: LitBoolType::False, .. })) = value.as_ref() else { return Ok(()) };
                        }
                        let assignable_idents = utils::fold_assignable_idents(assignable);
                        let condition_idents = utils::fold_expr_idents(expr);
                        if assignable_idents.iter().zip(condition_idents).any(|(a, b)| a.as_str() != b.as_str()) {
                            return Ok(());
                        }
                        loop_block_state.condition_updated = true;
                    }

                    Expr::Equal { lhs, rhs, .. } | Expr::NotEqual { lhs, rhs, .. } => {
                        // Don't check function applications or method calls
                        if matches!(lhs.as_ref(), Expr::FuncApp { .. } | Expr::MethodCall { .. }) || matches!(rhs.as_ref(), Expr::FuncApp { .. } | Expr::MethodCall { .. }) {
                            loop_block_state.condition_updated = true;
                            return Ok(());
                        }

                        let assignable_idents = utils::fold_assignable_idents(assignable);
                        let lhs_idents = utils::fold_expr_idents(lhs.as_ref());
                        let rhs_idents = utils::fold_expr_idents(rhs.as_ref());

                        if assignable_idents.iter().zip(lhs_idents).all(|(a, b)| a.as_str() == b.as_str()) {
                            loop_block_state.condition_updated = true;
                        } else if assignable_idents.iter().zip(rhs_idents).all(|(a, b)| a.as_str() == b.as_str()) {
                            loop_block_state.condition_updated = true;
                        }
                    }

                    Expr::LessThan { lhs, rhs, .. } | Expr::LessThanEq { lhs, rhs, .. } => {
                        // Check if `lhs` is incremented or `rhs` is decremented
                        match reassignment_op.variant {
                            ReassignmentOpVariant::Equals => {
                                let assignable_idents = utils::fold_assignable_idents(assignable);
                                let lhs_idents = utils::fold_expr_idents(lhs.as_ref());
                                let rhs_idents = utils::fold_expr_idents(rhs.as_ref());

                                if assignable_idents.iter().zip(lhs_idents).all(|(a, b)| a.as_str() == b.as_str()) {
                                    if matches!(value.as_ref(), Expr::Add { .. }) {
                                        loop_block_state.condition_updated = true;
                                    }
                                } else if assignable_idents.iter().zip(rhs_idents).all(|(a, b)| a.as_str() == b.as_str()) {
                                    if matches!(value.as_ref(), Expr::Sub { .. }) {
                                        loop_block_state.condition_updated = true;
                                    }
                                }
                            }

                            ReassignmentOpVariant::AddEquals => {
                                let assignable_idents = utils::fold_assignable_idents(assignable);
                                let condition_idents = utils::fold_expr_idents(lhs);
                                if assignable_idents.iter().zip(condition_idents).any(|(a, b)| a.as_str() != b.as_str()) {
                                    return Ok(());
                                }
                                loop_block_state.condition_updated = true;
                            }

                            ReassignmentOpVariant::SubEquals => {
                                let assignable_idents = utils::fold_assignable_idents(assignable);
                                let condition_idents = utils::fold_expr_idents(rhs);
                                if assignable_idents.iter().zip(condition_idents).any(|(a, b)| a.as_str() != b.as_str()) {
                                    return Ok(());
                                }
                                loop_block_state.condition_updated = true;
                            }

                            _ => {}
                        }
                    }

                    Expr::GreaterThan { lhs, rhs, .. } | Expr::GreaterThanEq { lhs, rhs, .. } => {
                        // Check if `lhs` is decremented or `rhs` is incremented
                        match reassignment_op.variant {
                            ReassignmentOpVariant::Equals => {
                                let assignable_idents = utils::fold_assignable_idents(assignable);
                                let lhs_idents = utils::fold_expr_idents(lhs.as_ref());
                                let rhs_idents = utils::fold_expr_idents(rhs.as_ref());

                                if assignable_idents.iter().zip(lhs_idents).all(|(a, b)| a.as_str() == b.as_str()) {
                                    if matches!(value.as_ref(), Expr::Sub { .. }) {
                                        loop_block_state.condition_updated = true;
                                    }
                                } else if assignable_idents.iter().zip(rhs_idents).all(|(a, b)| a.as_str() == b.as_str()) {
                                    if matches!(value.as_ref(), Expr::Add { .. }) {
                                        loop_block_state.condition_updated = true;
                                    }
                                }
                            }

                            ReassignmentOpVariant::AddEquals => {
                                let assignable_idents = utils::fold_assignable_idents(assignable);
                                let condition_idents = utils::fold_expr_idents(rhs);
                                if assignable_idents.iter().zip(condition_idents).any(|(a, b)| a.as_str() != b.as_str()) {
                                    return Ok(());
                                }
                                loop_block_state.condition_updated = true;
                            }

                            ReassignmentOpVariant::SubEquals => {
                                let assignable_idents = utils::fold_assignable_idents(assignable);
                                let condition_idents = utils::fold_expr_idents(lhs);
                                if assignable_idents.iter().zip(condition_idents).any(|(a, b)| a.as_str() != b.as_str()) {
                                    return Ok(());
                                }
                                loop_block_state.condition_updated = true;
                            }

                            _ => {}
                        }
                    }

                    _ => {}
                }
            }

            _ => {}
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_potential_infinite_loops() {
        crate::tests::test_detector("potential_infinite_loops")
    }
}
