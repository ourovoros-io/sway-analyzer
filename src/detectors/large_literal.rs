use crate::{
    error::Error,
    project::Project,
    report::Severity,
    utils,
    visitor::{AstVisitor, ExprContext},
};
use sway_ast::{Expr, Literal};
use sway_types::Spanned;

#[derive(Default)]
pub struct LargeLiteralVisitor;

impl AstVisitor for LargeLiteralVisitor {
    fn visit_expr(&mut self, context: &ExprContext, project: &mut Project) -> Result<(), Error> {
        let Expr::Literal(Literal::Int(i)) = context.expr else { return Ok(()) };

        let value = i.span.as_str();

        if value.starts_with("0x") || value.contains('_') || value.len() <= 6 {
            return Ok(());
        }

        let mut new_value = String::with_capacity(value.len() + value.len() / 3);
        
        for (idx, ch) in value.chars().rev().enumerate() {
            if idx != 0 && idx % 3 == 0 {
                new_value.push('_');
            }
    
            new_value.push(ch);
        }
    
        new_value = new_value.chars().rev().collect();

        project.report.borrow_mut().add_entry(
            context.path,
            project.span_to_line(context.path, &context.expr.span())?,
            Severity::Low,
            format!(
                "{} contains a large literal: `{value}`. Consider refactoring it to be more readable: `{new_value}`",
                utils::get_item_location(context.item, &context.item_impl, &context.item_fn),
            ),
        );
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_large_literal() {
        crate::tests::test_detector("large_literal", 7);
    }
}
