use crate::{
    error::Error,
    project::Project,
    utils,
    visitor::{
        AstVisitor, BlockContext, ExprContext, FnContext, ModuleContext, StatementContext,
        UseContext,
    },
    report::Severity,
};
use std::{collections::HashMap, path::PathBuf};
use sway_ast::Expr;
use sway_types::{Span, Spanned};

#[derive(Default)]
pub struct MissingLogsVisitor {
    module_states: HashMap<PathBuf, ModuleState>,
}

struct ModuleState {
    log_names: Vec<String>,
    fn_states: HashMap<Span, FnState>,
}

impl Default for ModuleState {
    fn default() -> Self {
        Self {
            // Since `std::logging::log` is part of the prelude, include it here
            log_names: vec!["log".into()],
            fn_states: Default::default(),
        }
    }
}

#[derive(Default)]
struct FnState {
    block_states: HashMap<Span, BlockState>,
}

#[derive(Default)]
struct BlockState {
    written: Vec<(Span, Span)>,
    logged: Vec<Span>,
}

impl AstVisitor for MissingLogsVisitor {
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

        // Check the use tree for `std::logging::log`
        if let Some(name) = utils::use_tree_to_name(&context.item_use.tree, "std::logging::log") {
            module_state.log_names.push(name);
        }

        Ok(())
    }

    fn visit_fn(&mut self, context: &FnContext, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Create the function state
        let fn_signature = context.item_fn.fn_signature.span();
        
        module_state.fn_states.entry(fn_signature).or_default();
        
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

        fn_state.block_states.entry(block_span).or_default();
        
        Ok(())
    }

    fn leave_block(&mut self, context: &BlockContext, project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get_mut(&fn_signature).unwrap();

        // Get the block state
        let block_span = context.block.span();
        let block_state = fn_state.block_states.get_mut(&block_span).unwrap();

        // Check each written storage variable to see if it has been logged
        for (storage_span, var_span) in block_state.written.iter() {
            if !block_state.logged.iter().any(|logged| {
                logged.as_str() == var_span.as_str() || logged.as_str() == format!("storage.{}.read()", storage_span.as_str())
            }) {
                project.report.borrow_mut().add_entry(
                    context.path,
                    project.span_to_line(context.path, storage_span)?,
                    Severity::Medium,
                    format!(
                        "{} writes to `storage.{}` without being logged.",
                        utils::get_item_location(context.item, &context.item_impl, &Some(context.item_fn)),
                        storage_span.as_str(),
                    ),
                );
            }
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
        let Some(block_span) = context.blocks.last() else { return Ok(()) };
        let block_state = fn_state.block_states.get_mut(block_span).unwrap();

        // Check for storage writes and add them to the block state
        if let Some((storage_name, var_name)) = utils::statement_to_storage_write_idents(context.statement) {
            block_state.written.push((storage_name.span(), var_name.span()));
        }

        Ok(())
    }

    fn visit_expr(&mut self, context: &ExprContext, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Get the function state
        let Some(item_fn) = context.item_fn.as_ref() else { return Ok(()) };
        let fn_signature = item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get_mut(&fn_signature).unwrap();

        // Get the block state
        let Some(block_span) = context.blocks.last() else { return Ok(()) };
        let block_state = fn_state.block_states.get_mut(block_span).unwrap();

        // Destructure the expression into a function application
        let Expr::FuncApp { func, args } = context.expr else { return Ok(()) };
        let Expr::Path(path) = func.as_ref() else { return Ok(()) };

        let log_args = utils::fold_punctuated(&args.inner);

        if log_args.len() != 1 {
            return Ok(());
        }

        let logged_span = log_args.last().unwrap().span();
        
        // Check for calls to the imported `log` function
        if path.suffix.is_empty() {
            for log_name in module_state.log_names.iter() {
                if path.prefix.name.as_str() == log_name {
                    // Add the `log` span to the block state
                    block_state.logged.push(logged_span);
                    break;
                }
            }
        }
        // Check for calls to the `std::logging::log` function
        else if path.suffix.len() == 2 {
            let "std" = path.prefix.name.as_str() else { return Ok(()) };
            let "logging" = path.suffix[0].1.name.as_str() else { return Ok(()) };
            let "log" = path.suffix[1].1.name.as_str() else { return Ok(()) };
            block_state.logged.push(logged_span);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_missing_logs() {
        crate::tests::test_detector("missing_logs", 2);
    }
}
