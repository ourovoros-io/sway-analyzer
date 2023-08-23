use crate::{
    error::Error,
    project::Project,
    report::Severity,
    visitor::{AstVisitor, ConfigurableContext, ExprContext},
};
use std::path::Path;
use sway_ast::{Expr, ItemConst, ItemImpl, ItemImplItem, ItemKind, Literal, Statement};
use sway_types::{Span, Spanned};

#[derive(Default)]
pub struct LargeLiteralsVisitor;

impl AstVisitor for LargeLiteralsVisitor {
    fn visit_configurable(&mut self, context: &ConfigurableContext, project: &mut Project) -> Result<(), Error> {
        // Case 1: Large Literal in Configurable
        for (a, _) in &context.item_configurable.fields.inner.value_separator_pairs {
            let Expr::Literal(Literal::Int(value)) = &a.value.initializer  else { return Ok(()) };

            if value.span().as_str().len() > 6 {
                add_report_entry(
                    project,
                    context.path,
                    value.span(),
                    "configurable",
                    None,
                    a.value.span().as_str(),
                    &a.value.span().as_str().replace(
                        value.span().as_str(),
                        format_large_number(value.span().as_str().parse::<u64>().unwrap()).as_str(),
                    ),
                )?;
            }
        }

        Ok(())
    }

    fn visit_expr(&mut self, context: &ExprContext, project: &mut Project) -> Result<(), Error> {
        match context.expr {
            // Case 1: Function call with large literal
            Expr::FuncApp { args, .. } => {
                for arg in args.inner.clone().into_iter() {
                    let Expr::Literal(i) = arg  else { continue };
                    let Ok(n) = i.span().as_str().parse::<u64>() else { return Ok(()) };
                    
                    if i.span().as_str().len() > 6 {
                        add_report_entry(
                            project,
                            context.path,
                            context.expr.span(),
                            "function",
                            Some(context.item_fn.unwrap().fn_signature.span().as_str()),
                            context.expr.span().as_str(),
                            &context
                                .expr
                                .span()
                                .as_str()
                                .replace(&n.to_string(), format_large_number(n).as_str()),
                        )?;
                    }
                }
            }

            // Case 2: Statement with large literal
            Expr::Literal(l) => {
                let Literal::Int(i) = l else { return Ok(()) };
                let Ok(n) = i.span().as_str().parse::<u64>() else { return Ok(()) };

                if n.to_string().len() > 6 {
                    // Case 2.1: Large literal statement in function
                    if context.item_fn.is_some() {
                        if let ItemKind::Const(ItemConst { .. }) = context.item {
                            add_report_entry(
                                project,
                                context.path,
                                context.expr.span(),
                                "function",
                                Some(context.item_fn.unwrap().fn_signature.span().as_str()),
                                context.item.span().as_str(),
                                &context
                                    .item
                                    .span()
                                    .as_str()
                                    .replace(&n.to_string(), format_large_number(n).as_str()),
                            )?;
                        } else if context.statement.is_some() {
                            let Statement::Let(statement) = context.statement.unwrap() else { return Ok(()) };

                            add_report_entry(
                                project,
                                context.path,
                                context.expr.span(),
                                "function",
                                Some(context.item_fn.unwrap().fn_signature.span().as_str()),
                                statement.span().as_str(),
                                &statement
                                    .span()
                                    .as_str()
                                    .replace(&n.to_string(), format_large_number(n).as_str()),
                            )?;
                        }
                    }
                    // Case 2.2: Large Literal statement in contract
                    else if let ItemKind::Impl(ItemImpl { contents, .. }) = context.item {
                        for c in &contents.inner {
                            let ItemImplItem::Const(ItemConst { ref expr_opt, .. }) = c.value else { return Ok(()) };

                            if expr_opt.as_ref().unwrap().span().as_str().len() > 6 {
                                add_report_entry(
                                    project,
                                    context.path,
                                    context.expr.span(),
                                    "contract",
                                    None,
                                    c.value.span().as_str(),
                                    &c.value
                                        .span()
                                        .as_str()
                                        .replace(&n.to_string(), format_large_number(n).as_str()),
                                )?;
                            }
                        }
                    } else {
                        add_report_entry(
                            project,
                            context.path,
                            context.expr.span(),
                            "contract",
                            None,
                            context.item.span().as_str(),
                            &context
                                .item
                                .span()
                                .as_str()
                                .replace(&n.to_string(), format_large_number(n).as_str()),
                        )?;
                    }
                }
            }

            _ => {}
        }

        Ok(())
    }
}

fn add_report_entry(
    project: &mut Project,
    path: &Path,
    span: Span,
    placement: &str,
    fn_name: Option<&str>,
    info: &str,
    solution: &str,
) -> Result<(), Error> {
    project.report.borrow_mut().add_entry(
        path,
        project.span_to_line(path, &span)?,
        Severity::Low,
        format!(
            "Found large literal in {} {} => `{}`. Consider refactoring it in order to be more readable: `{}`.",
            placement,
            fn_name.unwrap_or(""),
            info,
            solution,
        ),
    );
    Ok(())
}

/// Function to format large numbers with `_` separator
fn format_large_number(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::with_capacity(s.len() + s.len() / 3);

    for (idx, ch) in s.chars().rev().enumerate() {
        if idx != 0 && idx % 3 == 0 {
            result.push('_');
        }

        result.push(ch);
    }

    result.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Options;
    use std::path::PathBuf;

    #[test]
    fn test_large_literals() {
        let options = Options {
            directory: Some(PathBuf::from("test/large_literals")),
            detectors: vec!["large_literals".to_string()],
            ..Default::default()
        };

        let mut project = Project::try_from(&options).unwrap();
        project.analyze_modules().unwrap();

        println!("{project}");
    }
}
