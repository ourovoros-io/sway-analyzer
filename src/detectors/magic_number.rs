use crate::{
    error::Error,
    project::Project,
    scope::AstScope,
    utils,
    visitor::{AstVisitor, ExprContext, ModuleContext},
};
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    path::PathBuf,
    rc::Rc,
};
use sway_ast::{Expr, ItemKind, Literal};
use sway_types::{Span, Spanned};

#[derive(Default)]
pub struct MagicNumberVisitor {
    module_states: HashMap<PathBuf, ModuleState>,
}

#[derive(Default)]
struct ModuleState {
    visited_statements: HashSet<Span>,
}

impl AstVisitor for MagicNumberVisitor {
    fn visit_module(&mut self, context: &ModuleContext, _scope: Rc<RefCell<AstScope>>, _project: &mut Project) -> Result<(), Error> {
        // Create the module state
        if !self.module_states.contains_key(context.path) {
            self.module_states.insert(context.path.into(), ModuleState::default());
        }

        Ok(())
    }

    fn visit_expr(&mut self, context: &ExprContext, _scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        // Don't check constants
        if matches!(context.item, ItemKind::Const(_)) {
            return Ok(());
        }
        
        // Only check binary expressions
        let Some((lhs, rhs)) = utils::expr_binary_operands(context.expr) else { return Ok(()) };
        
        // Check if either `lhs` or `rhs` is a literal
        if !matches!(lhs, Expr::Literal(_)) && !matches!(rhs, Expr::Literal(_)) {
            return Ok(());
        }

        // Skip commonly-used values
        for x in [lhs, rhs] {
            if let Expr::Literal(Literal::Int(value)) = x {
                if value.parsed == 0u8.into() || value.parsed == 1u8.into() {
                    return Ok(());
                }
            }
        }

        // Only report the statement containing the expression once
        if let Some(statement) = context.statement.as_ref() {
            let module_state = self.module_states.get_mut(context.path).unwrap();

            if module_state.visited_statements.contains(&statement.span()) {
                return Ok(());
            }

            module_state.visited_statements.insert(statement.span());
        }

        project.report.borrow_mut().add_entry(
            context.path,
            project.span_to_line(context.path, &context.expr.span())?,
            crate::report::Severity::Low,
            format!(
                "{} contains magic number usage: `{}`. Consider introducing a constant value.",
                utils::get_item_location(context.item, &context.item_impl, &context.item_fn),
                context.expr.span().as_str(),
            ),
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_magic_number() {
        crate::tests::test_detector("magic_number", 5);
    }
}
