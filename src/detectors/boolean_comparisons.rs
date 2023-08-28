use crate::{
    error::Error,
    project::Project,
    report::Severity,
    utils,
    visitor::{AstVisitor, ExprContext, IfExprContext},
};
use std::path::Path;
use sway_ast::{Expr, ItemFn, ItemImpl, IfCondition};
use sway_types::Spanned;

#[derive(Default)]
pub struct BooleanComparisonsVisitor;

impl AstVisitor for BooleanComparisonsVisitor {
    fn visit_if_expr(&mut self, context: &IfExprContext, project: &mut Project) -> Result<(), Error> {
        let IfCondition::Expr(expr) = &context.if_expr.condition else { return Ok(()) };

        if utils::is_boolean_literal_or_negation(expr.as_ref()) {
            add_report_entry(project, context.path, expr, &context.item_impl, &Some(context.item_fn))?;
        }

        Ok(())
    }

    fn visit_expr(&mut self, context: &ExprContext, project: &mut Project) -> Result<(), Error> {
        match context.expr {
            Expr::Equal { lhs, rhs, .. } | Expr::NotEqual { lhs, rhs, .. } => {
                if !utils::is_boolean_literal_or_negation(lhs.as_ref()) && !utils::is_boolean_literal_or_negation(rhs.as_ref()) {
                    return Ok(());
                }
            }
        
            _ => return Ok(())
        }

        add_report_entry(project, context.path, context.expr, &context.item_impl, &context.item_fn)
    }
}

fn add_report_entry(project: &mut Project, path: &Path, expr: &Expr, item_impl: &Option<&ItemImpl>, item_fn: &Option<&ItemFn>) -> Result<(), Error> {
    project.report.borrow_mut().add_entry(
        path,
        project.span_to_line(path, &expr.span())?,
        Severity::Low,
        match item_fn.as_ref() {
            Some(item_fn) => {
                format!(
                    "The `{}` function contains a comparison with a boolean literal, which is unnecessary: `{}`",
                    if let Some(item_impl) = item_impl.as_ref() {
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
                    expr.span().as_str(),
                )
            }

            None => {
                format!(
                    "Found a comparison with a boolean literal, which is unnecessary: `{}`",
                    expr.span().as_str(),
                )
            }
        },
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Options;
    use std::path::PathBuf;

    #[test]
    fn test_boolean_comparisons() {
        let options = Options {
            directory: Some(PathBuf::from("test/boolean_comparisons")),
            detectors: vec!["boolean_comparisons".to_string()],
            ..Default::default()
        };

        let mut project = Project::try_from(&options).unwrap();
        project.analyze_modules().unwrap();

        println!("{project}");
    }
}
