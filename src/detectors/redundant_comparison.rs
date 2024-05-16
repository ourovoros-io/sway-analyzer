use crate::{
    error::Error,
    project::Project,
    report::Severity,
    scope::AstScope,
    utils,
    visitor::{
        AstVisitor, BlockContext, FnContext, IfExprContext, ModuleContext, StatementLetContext,
        WhileExprContext,
    },
};
use std::{cell::RefCell, collections::HashMap, path::PathBuf, rc::Rc};
use sway_ast::{Expr, IfCondition, Pattern};
use sway_types::{Span, Spanned};

#[derive(Default)]
pub struct RedundantComparisonVisitor {
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

impl FnState {
    fn get_expr_value(&mut self, expr: &Expr, blocks: &[Span]) -> Option<String> {
        match expr {
            Expr::Literal(literal) => Some(literal.span().str()),
            
            _ => {
                let idents = utils::fold_expr_ident_spans(expr);
                
                if idents.len() != 1 {
                    return None;
                }

                let var_name = idents[0].as_str();

                for block_span in blocks.iter().rev() {
                    let block_state = self.block_states.get(block_span).unwrap();

                    for var_state in block_state.var_states.iter().rev() {
                        if var_state.name == var_name {
                            return var_state.value.clone();
                        }
                    }
                }

                None
            }
        }
    }

    fn check_expr_for_redundant_comparisons(&mut self, expr: &Expr, blocks: &[Span]) -> Vec<Span> {
        let mut result = vec![];

        match expr {
            Expr::Equal { lhs, rhs, .. } |
            Expr::NotEqual { lhs, rhs, .. } |
            Expr::LessThan { lhs, rhs, .. } |
            Expr::GreaterThan { lhs, rhs, .. } |
            Expr::LessThanEq { lhs, rhs, .. } |
            Expr::GreaterThanEq { lhs, rhs, .. } => {
                let Some(_) = self.get_expr_value(lhs.as_ref(), blocks) else { return result };
                let Some(_) = self.get_expr_value(rhs.as_ref(), blocks) else { return result };
                result.push(expr.span());
            }
            
            Expr::LogicalAnd { lhs, rhs, .. } |
            Expr::LogicalOr { lhs, rhs, .. } => {
                result.extend(self.check_expr_for_redundant_comparisons(lhs.as_ref(), blocks));
                result.extend(self.check_expr_for_redundant_comparisons(rhs.as_ref(), blocks));
            }

            _ => {}
        }

        result
    }
}

#[derive(Default)]
struct BlockState {
    var_states: Vec<VarState>,
}

struct VarState {
    name: String,
    value: Option<String>,
}

impl AstVisitor for RedundantComparisonVisitor {
    fn visit_module(&mut self, context: &ModuleContext, _scope: Rc<RefCell<AstScope>>, _project: &mut Project) -> Result<(), Error> {
        // Create the module state
        if !self.module_states.contains_key(context.path) {
            self.module_states.insert(context.path.into(), ModuleState::default());
        }

        Ok(())
    }

    fn visit_fn(&mut self, context: &FnContext, _scope: Rc<RefCell<AstScope>>, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Create the function state
        let fn_signature = context.item_fn.fn_signature.span();
        
        module_state.fn_states.entry(fn_signature).or_default();
        
        Ok(())
    }

    fn visit_block(&mut self, context: &BlockContext, _scope: Rc<RefCell<AstScope>>, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get_mut(&fn_signature).unwrap();

        // Create the block state
        let block_span = context.block.span();

        fn_state.block_states.entry(block_span).or_default();

        Ok(())
    }

    fn visit_statement_let(&mut self, context: &StatementLetContext, _scope: Rc<RefCell<AstScope>>, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get_mut(&fn_signature).unwrap();

        // Get the block state
        let block_span = context.blocks.last().unwrap();
        let block_state = fn_state.block_states.get_mut(block_span).unwrap();

        // Create the var state(s)
        match &context.statement_let.pattern {
            Pattern::AmbiguousSingleIdent(ident) => {
                block_state.var_states.push(VarState {
                    name: ident.to_string(),
                    value: match &context.statement_let.expr {
                        Expr::Literal(literal) => Some(literal.span().str()),
                        _ => None,
                    },
                });
            }
            
            pattern => {
                for ident in utils::fold_pattern_idents(pattern) {
                    block_state.var_states.push(VarState {
                        name: ident.to_string(),
                        value: None,
                    });
                }
            }
        }

        Ok(())
    }

    fn visit_if_expr(&mut self, context: &IfExprContext, _scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get_mut(&fn_signature).unwrap();

        match &context.if_expr.condition {
            IfCondition::Expr(expr) => {
                for span in fn_state.check_expr_for_redundant_comparisons(expr, context.blocks.as_slice()) {
                    project.report.borrow_mut().add_entry(
                        context.path,
                        project.span_to_line(context.path, &span)?,
                        Severity::Low,
                        format!(
                            "{} contains a redundant comparison: `{}`",
                            utils::get_item_location(context.item, &context.item_impl, &Some(context.item_fn)),
                            span.as_str(),
                        ),
                    );
                }
            }

            IfCondition::Let { lhs, .. } => {
                // Get or create the if expression's body block state
                let block_span = context.if_expr.then_block.span();
                let block_state = fn_state.block_states.entry(block_span).or_default();

                // Create the var state(s)
                for ident in utils::fold_pattern_idents(lhs.as_ref()) {
                    block_state.var_states.push(VarState {
                        name: ident.to_string(),
                        value: None,
                    });
                }
            }
        }

        Ok(())
    }

    fn visit_while_expr(&mut self, context: &WhileExprContext, _scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get_mut(&fn_signature).unwrap();

        for span in fn_state.check_expr_for_redundant_comparisons(context.condition, context.blocks.as_slice()) {
            project.report.borrow_mut().add_entry(
                context.path,
                project.span_to_line(context.path, &span)?,
                Severity::Low,
                format!(
                    "{} contains a redundant comparison: `{}`",
                    utils::get_item_location(context.item, &context.item_impl, &Some(context.item_fn)),
                    span.as_str(),
                ),
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_redundant_comparison() {
        crate::tests::test_detector("redundant_comparison", 30);
    }
}
