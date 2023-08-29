use crate::{
    error::Error,
    project::Project,
    report::Severity,
    visitor::{AsmBlockContext, AstVisitor},
};
use sway_types::Spanned;

#[derive(Default)]
pub struct InlineAssemblyUsageVisitor;

impl AstVisitor for InlineAssemblyUsageVisitor {
    fn visit_asm_block(&mut self, context: &AsmBlockContext, project: &mut Project) -> Result<(), Error> {
        project.report.borrow_mut().add_entry(
            context.path,
            project.span_to_line(context.path, &context.asm.span())?,
            Severity::Medium,
            format!(
                "The `{}` function contains inline assembly usage.",
                if let Some(item_impl) = context.item_impl.as_ref() {
                    format!(
                        "{}::{}",
                        item_impl.ty.span().as_str(),
                        context.item_fn.fn_signature.name.as_str(),
                    )
                } else {
                    format!(
                        "{}",
                        context.item_fn.fn_signature.name.as_str(),
                    )
                }
            ),
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{project::Project, Options};
    use std::path::PathBuf;

    #[test]
    fn test_inline_assembly_usage() {
        let options = Options {
            directory: Some(PathBuf::from("test/inline_assembly_usage")),
            detectors: vec!["inline_assembly_usage".to_string()],
            ..Default::default()
        };

        let mut project = Project::try_from(&options).unwrap();
        project.analyze_modules().unwrap();

        println!("{project}");
    }
}
