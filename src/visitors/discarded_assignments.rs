use super::{AstVisitor, BlockContext, ExprContext, FnContext, ModuleContext, StatementContext, WhileExprContext};
use crate::{error::Error, project::Project, utils};
use std::{collections::HashMap, path::PathBuf};
use sway_ast::{expr::ReassignmentOpVariant, Expr, Statement, StatementLet, Pattern};
use sway_types::{Span, Spanned};

#[derive(Default)]
pub struct DiscardedAssignmentsVisitor {
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
    assignable_states: Vec<AssignableState>,
}

struct AssignableState {
    name: String,
    span: Span,
    used: bool,
    op: ReassignmentOpVariant,
}

impl AstVisitor for DiscardedAssignmentsVisitor {
    fn visit_module(&mut self, context: &ModuleContext,  _project: &mut Project) -> Result<(), Error> {
        // Create the module state
        if !self.module_states.contains_key(context.path) {
            self.module_states.insert(context.path.into(), ModuleState::default());
        }

        Ok(())
    }

    fn visit_fn(&mut self, context: &FnContext,  _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Create the function state
        let fn_signature = context.item_fn.fn_signature.span();

        if !module_state.fn_states.contains_key(&fn_signature) {
            module_state.fn_states.insert(fn_signature, FnState::default());
        }

        Ok(())
    }

    fn visit_block(&mut self, context: &BlockContext,  _project: &mut Project) -> Result<(), Error> {
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

        // Check for discarded assignments
        for assignable_state in block_state.assignable_states.iter() {
            if !assignable_state.used {
                project.report.borrow_mut().add_entry(
                    context.path,
                    project.span_to_line(context.path, &assignable_state.span)?,
                    format!("Assignment made to `{}` is discarded.", assignable_state.span.as_str()),
                );
            }
        }

        Ok(())
    }

    fn leave_while_expr(&mut self, context: &WhileExprContext, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get_mut(&fn_signature).unwrap();

        // Collect all identifier spans in `context.condition`
        let var_spans = utils::collect_ident_spans(context.condition);

        // Find the block state each variable state was declared in
        for var_span in var_spans {
            println!("checking while condition var: {}", var_span.as_str());

            for block_span in context.blocks.iter().rev() {
                // Get the block state
                let block_state = fn_state.block_states.get_mut(&block_span).unwrap();
        
                // Find the variable state and mark it as used
                if let Some(assignable_state) = block_state.assignable_states.iter_mut().find(|x| x.name == var_span.as_str()) {
                    println!("marking while condition var as used: {}", var_span.as_str());
                    assignable_state.used = true;
                    break;
                }
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
        let block_state = fn_state.block_states.get_mut(block_span).unwrap();

        // Create an assignment state for variable declarations
        if let Statement::Let(StatementLet {
            // TODO: handle other patterns
            pattern: Pattern::Var { name, .. },
            ..
        }) = context.statement {
            block_state.assignable_states.push(AssignableState {
                name: name.as_str().to_string(),
                span: name.span(),
                used: false,
                op: ReassignmentOpVariant::Equals,
            });
        }

        Ok(())
    }

    fn visit_expr(&mut self, context: &ExprContext, project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get_mut(&fn_signature).unwrap();

        let mut expr = context.expr;

        // If expr is an assignment, check if expr being assigned to was already assigned to in available block scopes
        if let Expr::Reassignment {
            assignable,
            reassignment_op,
            expr: value_expr,
        } = expr {
            let assignable_span = assignable.span();
            let mut assignable_state_exists = false;
            
            // Check if the assigned value has been used and create a report entry if not
            for block_span in context.blocks.iter().rev() {
                // Get the block state
                let block_state = fn_state.block_states.get_mut(block_span).unwrap();

                // Check if the assignable state exists
                if let Some(assignable_state) = block_state.assignable_states.iter_mut().find(|x| x.name == assignable_span.as_str()) {
                    // Check for assignment invariants
                    let assignment_discarded = match (&reassignment_op.variant, &assignable_state.op) {
                        (ReassignmentOpVariant::Equals, ReassignmentOpVariant::AddEquals) |
                        (ReassignmentOpVariant::Equals, ReassignmentOpVariant::SubEquals) |
                        (ReassignmentOpVariant::Equals, ReassignmentOpVariant::MulEquals) |
                        (ReassignmentOpVariant::Equals, ReassignmentOpVariant::DivEquals) |
                        (ReassignmentOpVariant::Equals, ReassignmentOpVariant::ShlEquals) |
                        (ReassignmentOpVariant::Equals, ReassignmentOpVariant::ShrEquals) => !assignable_state.used,
                        _ => false,
                    };

                    // If the assigned value has not been used, create a report entry
                    if !assignable_state.used && assignment_discarded {
                        project.report.borrow_mut().add_entry(
                            context.path,
                            project.span_to_line(context.path, &assignable_state.span)?,
                            format!(
                                "Assignment made to `{}` is discarded by the assignment made on L{}.",
                                assignable_state.span.as_str(),
                                project.span_to_line(context.path, &assignable_span)?.unwrap(),
                            ),
                        );
                    }

                    // Since the assignable has been assigned a new value, update its span and mark it as unused
                    assignable_state.span = assignable_span.clone();
                    assignable_state.used = false;
                    assignable_state.op = reassignment_op.variant.clone();

                    // Note that the assignable state exists and stop the lookup
                    assignable_state_exists = true;
                    break;
                }
            }

            // If the assignable state does not exist, create a new assignable state in the current block state
            if !assignable_state_exists {
                // Get the current block state
                let block_span = context.blocks.last().unwrap();
                let block_state = fn_state.block_states.get_mut(block_span).unwrap();
    
                // Create a new assignable state
                block_state.assignable_states.push(AssignableState {
                    name: assignable_span.as_str().to_string(),
                    span: assignable_span,
                    used: false,
                    op: reassignment_op.variant.clone(),
                });
            }

            expr = value_expr.as_ref();
        }

        // Collect all identifier spans in the expression
        let ident_spans = utils::collect_ident_spans(expr);

        // If an assignable state for any of the identifier spans exists in any of the current blocks, mark it as used
        for ident_span in ident_spans.into_iter() {
            for block_span in context.blocks.iter().rev() {
                // Get the block state
                let block_state = fn_state.block_states.get_mut(block_span).unwrap();

                // If the assignable state is a direct match, mark it as used
                if let Some(assignable_state) = block_state.assignable_states.iter_mut().find(|x| x.name == ident_span.as_str()) {
                    assignable_state.used = true;
                }

                // If the identifier span is a higher level variable, but fields of it were updated, mark all of their assignable states as used
                for assignable_state in block_state.assignable_states.iter_mut().filter(|x| x.name.starts_with(format!("{}.", ident_span.as_str()).as_str())) {
                    assignable_state.used = true;
                }
            }
        }

        Ok(())
    }
}
