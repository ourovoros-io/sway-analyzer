use super::{AstVisitor, StorageFieldContext};
use crate::{error::Error, project::Project};
use sway_types::Spanned;

pub struct StorageFieldsVisitor;

impl AstVisitor for StorageFieldsVisitor {
    fn visit_storage_field(
        &mut self,
        context: &StorageFieldContext,
        project: &mut Project,
    ) -> Result<(), Error> {
        project.report.borrow_mut().add_entry(
            context.path,
            project.span_to_line(context.path, &context.field.name.span())?,
            format!("Found storage field: {}", context.field.name.as_str()),
        );

        Ok(())
    }
}
