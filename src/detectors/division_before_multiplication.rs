use crate::{
    error::Error,
    project::Project,
    report::Severity,
    visitor::{
        AstVisitor, BlockContext, ConfigurableContext, ConstContext, ExprContext, FnContext,
        ModuleContext, StatementContext,
    },
};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use sway_ast::{Expr, Statement, StatementLet};
use sway_types::{Span, Spanned};

#[derive(Default)]
pub struct DivisionBeforeMultiplicationVisitor {
    module_states: HashMap<PathBuf, ModuleState>,
}

#[derive(Default)]
struct ModuleState {
    fn_states: HashMap<Span, FnState>,
}

#[derive(Default)]
struct FnState {
    block_states: HashMap<Span, BlockState>,
}

#[derive(Default)]
struct BlockState {
    variable_states: HashMap<Span, bool>,
}

impl AstVisitor for DivisionBeforeMultiplicationVisitor {
    fn visit_module(&mut self, context: &ModuleContext, _project: &mut Project) -> Result<(), Error> {
        // Create the module state
        if !self.module_states.contains_key(context.path) {
            self.module_states.insert(context.path.into(), ModuleState::default());
        }

        Ok(())
    }

    fn visit_fn(&mut self, context: &FnContext, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Create the function state
        let fn_signature = context.item_fn.fn_signature.span();
        
        if !module_state.fn_states.contains_key(&fn_signature) {
            module_state.fn_states.insert(fn_signature, FnState::default());
        }
        
        Ok(())
    }

    fn visit_block(&mut self, context: &BlockContext, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get_mut(&fn_signature).unwrap();

        // Create the block state
        let block_span = context.block.span();

        if !fn_state.block_states.contains_key(&block_span) {
            fn_state.block_states.insert(block_span, BlockState::default());
        }
        
        Ok(())
    }

    fn visit_statement(&mut self, context: &StatementContext, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get_mut(&fn_signature).unwrap();

        // Get the block state
        let block_span = context.blocks.last().unwrap();
        let block_state = fn_state.block_states.get_mut(block_span).unwrap();

        if let Statement::Let(StatementLet { pattern, expr: Expr::Div { .. }, .. }) = context.statement {
            block_state.variable_states.insert(pattern.span(), true);
        }

        Ok(())
    }

    fn visit_expr(&mut self, context: &ExprContext, project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get(context.path).unwrap();

        // Get the function state
        let Some(item_fn) = context.item_fn.as_ref() else { return Ok(()) };
        let fn_signature = item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get(&fn_signature).unwrap();

        let Expr::Mul { lhs, .. } = context.expr else { return Ok(()) };

        'var_lookup: for block_span in context.blocks.iter().rev() {
            let block_state = fn_state.block_states.get(block_span).unwrap();

            for (k, _) in block_state.variable_states.iter() {
                if k.as_str() == lhs.span().as_str() {
                    add_report_entry(
                        project,
                        context.path,
                        context.expr.span(),
                        "function",
                        Some(context.item_fn.unwrap().fn_signature.span().as_str()),
                        context.statement.unwrap().span().as_str(),
                    )?;
                    break 'var_lookup;
                }
            }    
        }

        match *lhs.to_owned() {
            Expr::Parens(expr) => {
                let Expr::Div { .. } = expr.inner.as_ref() else { return Ok(()) };

                // Check if we are in a function or not
                if context.statement.is_some() {
                    add_report_entry(
                        project,
                        context.path,
                        context.expr.span(),
                        "function",
                        Some(context.item_fn.unwrap().fn_signature.span().as_str()),
                        context.statement.unwrap().span().as_str(),
                    )?;
                }
            }

            Expr::Div { .. } => {
                // Check if we are in a function or not
                if context.statement.is_some() {
                    add_report_entry(
                        project,
                        context.path,
                        context.expr.span(),
                        "function",
                        Some(context.item_fn.unwrap().fn_signature.span().as_str()),
                        context.statement.unwrap().span().as_str(),
                    )?;
                }
            }

            _ => {}
        }

        Ok(())
    }

    fn visit_const(&mut self, context: &ConstContext, project: &mut Project) -> Result<(), Error> {
        let Some(expr) = context.item_const.expr_opt.as_ref() else { return Ok(()) };
        let Expr::Mul { lhs, .. } = expr else { return Ok(()) };

        match *lhs.to_owned() {
            Expr::Parens(expr) => {
                let Expr::Div { .. } = expr.inner.as_ref() else { return Ok(()) };

                add_report_entry(
                    project,
                    context.path,
                    expr.span(),
                    "constant",
                    None,
                    context.item_const.span().as_str(),
                )?;
            }

            Expr::Div { .. } => {
                add_report_entry(
                    project,
                    context.path,
                    expr.span(),
                    "constant",
                    None,
                    context.item_const.span().as_str(),
                )?;
            }
            _ => {}
        }

        Ok(())
    }

    fn visit_configurable(&mut self, context: &ConfigurableContext, project: &mut Project) -> Result<(), Error> {
        for a in &context.item_configurable.fields.inner {
            let Expr::Mul { lhs, .. } = &a.value.initializer else { continue };

            match lhs.as_ref() {
                Expr::Parens(paren_expr) => {
                    let Expr::Div { .. } = *paren_expr.inner else {continue};
                    add_report_entry(
                        project,
                        context.path,
                        a.value.initializer.span(),
                        "configurable",
                        None,
                        a.value.span().as_str(),
                    )?;
                }

                Expr::Div { .. } => {
                    add_report_entry(
                        project,
                        context.path,
                        a.value.initializer.span(),
                        "configurable",
                        None,
                        a.value.span().as_str(),
                    )?;
                }

                _ => {}
            }
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
) -> Result<(), Error> {
    project.report.borrow_mut().add_entry(
        path,
        project.span_to_line(path, &span)?,
        Severity::Medium,
        format!(
            "Found division before multiplication in {} {} => `{}`. Consider ordering multiplication before division.",
            placement,
            fn_name.unwrap_or(""),
            info
        ),
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{project::Project, Options};
    use std::path::PathBuf;

    #[test]
    fn test_division_before_multiplication() {
        let options = Options {
            directory: Some(PathBuf::from("test/division_before_multiplication")),
            detectors: vec!["division_before_multiplication".to_string()],
            ..Default::default()
        };

        let mut project = Project::try_from(&options).unwrap();
        project.analyze_modules().unwrap();

        println!("{project}");
    }
}
