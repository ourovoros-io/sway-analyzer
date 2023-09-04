use crate::{
    error::Error,
    project::Project,
    report::Severity,
    visitor::{
        AstVisitor, ConfigurableContext, ExprContext, ModuleContext, StatementContext, UseContext,
    },
};
use std::{collections::HashMap, path::PathBuf};
use sway_ast::{
    attribute::Annotated, Expr, ItemConst, ItemKind, PathExpr, Statement, StatementLet, UseTree,
};
use sway_types::{Span, Spanned};

#[derive(Default)]
pub struct UnusedImportsVisitor {
    module_states: HashMap<PathBuf, ModuleState>,
}

#[derive(Default)]
struct ModuleState {
    imports: Vec<String>,
    usage_states: HashMap<String, u32>,
    span_usage_states: HashMap<String, Span>,
}

impl AstVisitor for UnusedImportsVisitor {
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

    fn visit_statement(&mut self, context: &StatementContext, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        match context.statement {
            Statement::Let(StatementLet { expr, .. }) => {
                let Some(matched) = module_state.span_usage_states.get(expr.span().as_str()) else { return Ok(()) };

                module_state
                    .usage_states
                    .entry(matched.as_str().to_owned())
                    .and_modify(|counter| *counter += 1)
                    .or_insert(0);
            }
            
            Statement::Item(Annotated {
                value: ItemKind::Const(ItemConst { expr_opt, .. }),
                ..
            }) => {
                let Some(matched) = module_state.span_usage_states.get(expr_opt.as_ref().unwrap().span().as_str()) else { return Ok(()) };

                module_state
                    .usage_states
                    .entry(matched.as_str().to_owned())
                    .and_modify(|counter| *counter += 1)
                    .or_insert(0);
            }

            Statement::Expr { expr, .. } => {
                let Expr::FuncApp { func, args } = expr else { return Ok(()) };

                if let Some(matched) = module_state.span_usage_states.get(func.span().as_str()) {
                    module_state
                        .usage_states
                        .entry(matched.as_str().to_owned())
                        .and_modify(|counter| *counter += 1)
                        .or_insert(0);
                }
                
                for arg in &args.inner {
                    if let Some(matched) = module_state.span_usage_states.get(arg.span().as_str()) {
                        module_state
                            .usage_states
                            .entry(matched.as_str().to_owned())
                            .and_modify(|counter| *counter += 1)
                            .or_insert(0);
                    }
                }
            }

            _ => {}
        }

        Ok(())
    }

    fn visit_expr(&mut self, context: &ExprContext, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        let Expr::Path(PathExpr{ prefix, ..  }) = context.expr else { return Ok(()) };
        let Some(matched) = module_state.span_usage_states.get(&prefix.span().str()) else { return Ok(()) };

        module_state
            .usage_states
            .entry(matched.as_str().to_owned())
            .and_modify(|counter| *counter += 1)
            .or_insert(0);

        Ok(())
    }

    fn visit_configurable(&mut self, context: &ConfigurableContext, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        for a in &context.item_configurable.fields.inner {
            let Some(matched) = module_state.span_usage_states.get(&a.value.initializer.span().str()) else { return Ok(()) };
            
            module_state
                .usage_states
                .entry(matched.as_str().to_owned())
                .and_modify(|counter| *counter += 1)
                .or_insert(0);
        }

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
    fn test_redundant_import() {
        crate::tests::test_detector("redundant_import")
    }
}
