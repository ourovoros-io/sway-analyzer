use crate::{
    error::Error,
    project::Project,
    report::Severity,
    scope::AstScope,
    utils,
    visitor::{AstVisitor, WhileExprContext},
};
use std::{cell::RefCell, rc::Rc};
use sway_ast::Expr;
use sway_types::Spanned;

#[derive(Default)]
pub struct StorageReadInLoopConditionVisitor;

impl AstVisitor for StorageReadInLoopConditionVisitor {
    fn visit_while_expr(&mut self, context: &WhileExprContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        fn find_storage_read(expr: &Expr, context: &WhileExprContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
            match expr {
                Expr::Mul { lhs, rhs, .. } |
                Expr::Div { lhs, rhs, .. } |
                Expr::Pow { lhs, rhs, .. } |
                Expr::Modulo { lhs, rhs, .. } |
                Expr::Add { lhs, rhs, .. } |
                Expr::Sub { lhs, rhs, .. } |
                Expr::Shl { lhs, rhs, .. } |
                Expr::Shr { lhs, rhs, .. } |
                Expr::BitAnd { lhs, rhs, .. } |
                Expr::BitXor { lhs, rhs, .. } |
                Expr::BitOr { lhs, rhs, .. } |
                Expr::Equal { lhs, rhs, .. } |
                Expr::NotEqual { lhs, rhs, .. } |
                Expr::LessThan { lhs, rhs, .. } |
                Expr::GreaterThan { lhs, rhs, .. } |
                Expr::LessThanEq { lhs, rhs, .. } |
                Expr::GreaterThanEq { lhs, rhs, .. } |
                Expr::LogicalAnd { lhs, rhs, .. } |
                Expr::LogicalOr { lhs, rhs, .. } => {
                    find_storage_read(lhs.as_ref(), context, scope.clone(), project)?;
                    find_storage_read(rhs.as_ref(), context, scope.clone(), project)?;
                }
                
                _ => {
                    let storage_idents = utils::fold_expr_idents(expr);
                
                    if storage_idents.len() < 3 {
                        return Ok(());
                    }
                
                    if storage_idents[0].as_str() != "storage" {
                        return Ok(());
                    }
                
                    if storage_idents.last().unwrap().as_str() != "len" {
                        return Ok(());
                    }
                    
                    project.report.borrow_mut().add_entry(
                        context.path,
                        project.span_to_line(context.path, &expr.span())?,
                        Severity::Low,
                        format!(
                            "The {} contains a loop with a condition that depends on a storage read: `{}`. Consider storing the expression in a local variable in order to reduce gas costs.",
                            utils::get_item_location(context.item, &context.item_impl, &Some(context.item_fn)),
                            expr.span().as_str(),
                        ),
                    );
                }
            }

            Ok(())
        }

        find_storage_read(context.condition, context, scope.clone(), project)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_storage_read_in_loop_condition() {
        crate::tests::test_detector("storage_read_in_loop_condition", 2);
    }
}
