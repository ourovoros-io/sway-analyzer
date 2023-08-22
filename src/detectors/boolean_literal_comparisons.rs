use crate::{
    error::Error,
    project::Project,
    report::Severity,
    visitor::{AstVisitor, ExprContext},
};
use sway_ast::Expr;
use sway_types::Spanned;

#[derive(Default)]
pub struct BooleanLiteralComparisonsVisitor;

impl AstVisitor for BooleanLiteralComparisonsVisitor {
    fn visit_expr(&mut self, context: &ExprContext, project: &mut Project) -> Result<(), Error> {
        match context.expr {
            Expr::Equal { lhs, rhs, .. } | Expr::NotEqual { lhs, rhs, .. } => {
                if let Expr::Literal(lhs) = lhs.as_ref() {
                    let ("true" | "false") = lhs.span().as_str() else { return Ok(()) };
                } else if let Expr::Literal(rhs) = rhs.as_ref() {
                    let ("true" | "false") = rhs.span().as_str() else { return Ok(()) };
                } else {
                    return Ok(());
                }
            }
        
            _ => return Ok(())
        }

        project.report.borrow_mut().add_entry(
            context.path,
            project.span_to_line(context.path, &context.expr.span())?,
            Severity::Low,
            match context.item_fn.as_ref() {
                Some(item_fn) => {
                    format!(
                        "The `{}` function contains a comparison with a boolean literal, which is unnecessary: `{}`",
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
                    )
                }

                None => {
                    format!(
                        "Found a comparison with a boolean literal, which is unnecessary: `{}`",
                        context.expr.span().as_str(),
                    )
                }
            },
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Options;
    use std::path::PathBuf;

    #[test]
    fn test_boolean_literal_comparisons() {
        let options = Options {
            directory: Some(PathBuf::from("test/boolean_literal_comparisons")),
            detectors: vec!["boolean_literal_comparisons".to_string()],
            ..Default::default()
        };

        let mut project = Project::try_from(&options).unwrap();
        project.analyze_modules().unwrap();

        println!("{project}");
    }
}
