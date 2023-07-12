use super::{AsmBlockContext, AstVisitor};
use crate::{error::Error, project::Project};
use sway_types::Spanned;

#[derive(Default)]
pub struct InlineAssemblyUsageVisitor;

impl AstVisitor for InlineAssemblyUsageVisitor {
    fn visit_asm_block(&mut self, context: &AsmBlockContext, project: &mut Project) -> Result<(), Error> {
        project.report.borrow_mut().add_entry(
            context.path,
            project.span_to_line(context.path, &context.asm.span())?,
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
