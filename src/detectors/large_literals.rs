use crate::{
    error::Error,
    project::Project,
    report::Severity,
    visitor::{AstVisitor, ExprContext},
};
use sway_ast::{Expr, ItemKind, Literal};
use sway_types::Spanned;

#[derive(Default)]
pub struct LargeLiteralsVisitor;

impl AstVisitor for LargeLiteralsVisitor {
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
                match context.item {
                    ItemKind::Fn(item_fn) => if let Some(item_impl) = context.item_impl.as_ref() {
                        format!(
                            "The `{}::{}` function",
                            item_impl.ty.span().as_str(),
                            item_fn.fn_signature.name.as_str(),
                        )
                    } else {
                        format!(
                            "The `{}` function",
                            item_fn.fn_signature.name.as_str(),
                        )
                    },

                    ItemKind::Const(item_const) => match (context.item_impl.as_ref(), context.item_fn.as_ref()) {
                        (Some(item_impl), Some(item_fn)) => format!(
                            "The `{}` constant in the `{}::{}` function",
                            item_const.name,
                            item_impl.ty.span().as_str(),
                            item_fn.fn_signature.name,
                        ),

                        (Some(item_impl), None) => format!(
                            "The `{}::{}` constant",
                            item_impl.ty.span().as_str(),
                            item_const.name.as_str(),
                        ),

                        (None, Some(item_fn)) => format!(
                            "The `{}` constant in the `{}` function",
                            item_const.name,
                            item_fn.fn_signature.name,
                        ),

                        (None, None) => format!(
                            "The `{}` constant",
                            item_const.name,
                        ),
                    },

                    ItemKind::Storage(_) => format!("Storage"),
                    ItemKind::Configurable(_) => format!("Configurable"),
                    
                    _ => panic!("Unhandled large literals scope: {:#?}", context.item),
                },
            ),
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
