use crate::{
    error::Error,
    project::Project,
    report::Severity,
    utils,
    visitor::{
        AstVisitor, ConfigurableFieldContext, ExprContext, FnContext, ModuleContext,
        StatementContext, UseContext,
    },
};
use std::{collections::HashMap, path::PathBuf};
use sway_ast::{
    attribute::Annotated, Expr, FnArgs, ItemConst, ItemKind, PathExpr, Pattern, Statement,
    StatementLet, UseTree,
};
use sway_types::{Span, Spanned};

#[derive(Default)]
pub struct UnusedImportVisitor {
    module_states: HashMap<PathBuf, ModuleState>,
}

#[derive(Default)]
struct ModuleState {
    imports: Vec<String>,
    usage_states: HashMap<String, u32>,
    span_usage_states: HashMap<String, Span>,
}

impl ModuleState {
    fn check_span_usage(&mut self, span: &Span) {
        let Some(matched) = self.span_usage_states.get_mut(span.as_str()) else { return };

        self.usage_states
            .entry(matched.as_str().to_owned())
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    fn check_expr_usage(&mut self, expr: &Expr) {
        utils::map_expr(expr, &mut |expr| {
            if let Expr::Path(PathExpr { prefix, .. }) = expr {
                self.check_span_usage(&prefix.span());
            }
        });
    }

    fn check_pattern_usage(&mut self, pattern: &Pattern) {
        utils::map_pattern(pattern, &mut |pattern| {
            if let Pattern::Constructor { path, .. } | Pattern::Struct { path, .. } = pattern {
                self.check_span_usage(&path.span());
            }
        });
    }
}

impl AstVisitor for UnusedImportVisitor {
    fn visit_module(&mut self, context: &ModuleContext, _project: &mut Project) -> Result<(), Error> {
        // Create the module state
        if !self.module_states.contains_key(context.path) {
            self.module_states.insert(context.path.into(), ModuleState::default());
        }

        Ok(())
    }

    fn visit_use(&mut self, context: &UseContext, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Destructure the use tree
        let UseTree::Path { suffix, .. } = &context.item_use.tree else { return Ok(()) };
        let UseTree::Path { suffix, .. } = suffix.as_ref() else { return Ok(()) };
        
        if let UseTree::Group { imports } = suffix.as_ref() {
            for import in &imports.inner {
                let UseTree::Name { name } = import else { return Ok(()) };
                module_state.usage_states.insert(name.span().str(), 0);
                module_state.span_usage_states.insert(name.span().str(), name.span());
                module_state.imports.push(name.span().str());
            }
        } else {
            let UseTree::Name { name } = *suffix.to_owned() else { return Ok(()) };
            module_state.usage_states.insert(name.span().str(), 0);
            module_state.imports.push(name.span().str());
            module_state.span_usage_states.insert(name.span().str(), name.span());
        }

        Ok(())
    }

    fn visit_fn(&mut self, context: &FnContext, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        let args = match &context.item_fn.fn_signature.arguments.inner {
            FnArgs::Static(args) => args,
            FnArgs::NonStatic { args_opt: Some(args), .. } => &args.1,
            _ => return Ok(()),
        };

        for arg in args {
            module_state.check_pattern_usage(&arg.pattern);
            module_state.check_span_usage(&arg.ty.span());
        }

        Ok(())
    }

    fn visit_statement(&mut self, context: &StatementContext, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        match context.statement {
            Statement::Let(StatementLet { pattern, ty_opt, expr, .. }) => {
                module_state.check_pattern_usage(pattern);

                if let Some((_, ty)) = ty_opt.as_ref() {
                    module_state.check_span_usage(&ty.span());
                }

                module_state.check_expr_usage(expr);
            }
            
            Statement::Item(Annotated {
                value: ItemKind::Const(ItemConst { ty_opt, expr_opt, .. }),
                ..
            }) => {
                if let Some((_, ty)) = ty_opt.as_ref() {
                    module_state.check_span_usage(&ty.span());
                }

                if let Some(expr) = expr_opt.as_ref() {
                    module_state.check_expr_usage(expr);
                }
            }

            _ => {}
        }

        Ok(())
    }

    fn visit_expr(&mut self, context: &ExprContext, _project: &mut Project) -> Result<(), Error> {
        let module_state = self.module_states.get_mut(context.path).unwrap();
        module_state.check_expr_usage(context.expr);
        Ok(())
    }

    fn visit_configurable_field(&mut self, context: &ConfigurableFieldContext, _project: &mut Project) -> Result<(), Error> {
        let module_state = self.module_states.get_mut(context.path).unwrap();
        module_state.check_span_usage(&context.field.ty.span());
        module_state.check_expr_usage(&context.field.initializer);
        Ok(())
    }

    fn leave_module(&mut self, context: &ModuleContext, project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        for (name, count) in &module_state.usage_states {
            if *count == 0 {
                project.report.borrow_mut().add_entry(
                    context.path,
                    project.span_to_line(context.path, &module_state.span_usage_states.get(name).unwrap())?,
                    Severity::Low,
                    format!("Found unused import: `{name}`. Consider removing any unused imports."),
                );
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_unused_import() {
        crate::tests::test_detector("unused_import", 2);
    }
}
