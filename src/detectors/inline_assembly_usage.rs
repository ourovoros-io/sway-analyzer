use crate::{
    error::Error,
    project::Project,
    report::Severity,
    utils,
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
                "{} contains inline assembly usage.",
                utils::get_item_location(context.item, &context.item_impl, &Some(context.item_fn)),
            ),
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_inline_assembly_usage() {
        crate::tests::test_detector("inline_assembly_usage")
    }
}
