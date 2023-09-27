use crate::{visitor::{AstVisitor, FnContext}, project::Project, error::Error, report::Severity, utils};
use sway_ast::{Statement, Expr};
use sway_types::Spanned;

#[derive(Default)]
pub struct ExplicitReturnStatementVisitor;

impl AstVisitor for ExplicitReturnStatementVisitor {
    fn visit_fn(&mut self, context: &FnContext, project: &mut Project) -> Result<(), Error> {
        if let Some(expr) = context.item_fn.body.inner.final_expr_opt.as_ref().map(Box::as_ref) {
            let Expr::Return { expr_opt, .. } = expr else { return Ok(()) };
            
            project.report.borrow_mut().add_entry(
                context.path,
                project.span_to_line(context.path, &expr.span())?,
                Severity::Low,
                if let Some(expr) = expr_opt.as_ref().map(Box::as_ref) {
                    format!(
                        "The {} contains an explicit return expression, which is unnecessary. Consider replacing `return {}` with `{}`.",
                        utils::get_item_location(context.item, &context.item_impl, &Some(context.item_fn)),
                        expr.span().as_str(),
                        expr.span().as_str(),
                    )
                } else {
                    format!(
                        "The {} contains an explicit return expression, which is unnecessary. Consider removing `return`.",
                        utils::get_item_location(context.item, &context.item_impl, &Some(context.item_fn)),
                    )
                },
            );
        }
        else if let Some(statement) = context.item_fn.body.inner.statements.last() {
            let Statement::Expr { expr: Expr::Return { expr_opt, .. }, .. } = statement else { return Ok(()) };

            project.report.borrow_mut().add_entry(
                context.path,
                project.span_to_line(context.path, &statement.span())?,
                Severity::Low,
                if let Some(expr) = expr_opt.as_ref().map(Box::as_ref) {
                    format!(
                        "The {} contains an explicit return statement, which is unnecessary. Consider replacing `return {};` with `{}`.",
                        utils::get_item_location(context.item, &context.item_impl, &Some(context.item_fn)),
                        expr.span().as_str(),
                        expr.span().as_str(),
                    )
                } else {
                    format!(
                        "The {} contains an explicit return statement, which is unnecessary. Consider removing `return;`.",
                        utils::get_item_location(context.item, &context.item_impl, &Some(context.item_fn)),
                    )
                },
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_explicit_return_statement() {
        crate::tests::test_detector("explicit_return_statement");
    }
}
