use crate::{
    error::Error,
    project::Project,
    report::Severity,
    scope::AstScope,
    utils,
    visitor::{AstVisitor, ExprContext, IfExprContext},
};
use std::{cell::RefCell, path::Path, rc::Rc};
use sway_ast::{Expr, IfCondition, ItemFn, ItemImpl, ItemKind};
use sway_types::Spanned;

#[derive(Default)]
pub struct BooleanComparisonVisitor;

impl AstVisitor for BooleanComparisonVisitor {
    fn visit_if_expr(&mut self, context: &IfExprContext, _scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        let IfCondition::Expr(expr) = &context.if_expr.condition else { return Ok(()) };

        if utils::is_boolean_literal_or_negation(expr.as_ref()) {
            add_report_entry(project, context.path, expr, context.item, &context.item_impl, &Some(context.item_fn))?;
        }

        Ok(())
    }

    fn visit_expr(&mut self, context: &ExprContext, _scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        let (Expr::Equal { lhs, rhs, .. } | Expr::NotEqual { lhs, rhs, .. }) = context.expr else { return Ok(()) };
        
        if !utils::is_boolean_literal_or_negation(lhs.as_ref()) && !utils::is_boolean_literal_or_negation(rhs.as_ref()) {
            return Ok(());
        }

        add_report_entry(project, context.path, context.expr, context.item, &context.item_impl, &context.item_fn)
    }
}

fn add_report_entry(project: &mut Project, path: &Path, expr: &Expr, item: &ItemKind, item_impl: &Option<&ItemImpl>, item_fn: &Option<&ItemFn>) -> Result<(), Error> {
    project.report.borrow_mut().add_entry(
        path,
        project.span_to_line(path, &expr.span())?,
        Severity::Low,
        format!(
            "{} contains a comparison with a boolean literal, which is unnecessary: `{}`",
            utils::get_item_location(item, item_impl, item_fn),
            expr.span().as_str(),
        ),
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_boolean_comparison() {
        crate::tests::test_detector("boolean_comparison", 10);
    }
}
