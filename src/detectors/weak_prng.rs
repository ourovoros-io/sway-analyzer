use crate::{
    error::Error,
    project::Project,
    visitor::{
        AstVisitor, BlockContext, ExprContext, FnContext, ModuleContext, StatementContext,
        StatementLetContext, UseContext,
    }, report::Severity, utils,
};
use std::{collections::HashMap, path::PathBuf};
use sway_ast::{Assignable, Expr, Pattern, Statement};
use sway_types::{Span, Spanned};

#[derive(Default)]
pub struct WeakPrngVisitor {
    module_states: HashMap<PathBuf, ModuleState>,
}

#[derive(Default)]
struct ModuleState {
    timestamp_names: Vec<String>,
    timestamp_of_block_names: Vec<String>,
    fn_states: HashMap<Span, FnState>,
}

impl ModuleState {
    fn expr_is_timestamp_call(&self, expr: &Expr) -> bool {
        // Destructure the expression into a function application
        let Expr::FuncApp { func, .. } = expr else { return false };
        let Expr::Path(path) = func.as_ref() else { return false };
        
        // Update the variable state
        match path.span().as_str() {
            "std::block::timestamp" => true,
            "std::block::timestamp_of_block" => true,
            s if self.timestamp_names.iter().any(|name| s == name) => true,
            s if self.timestamp_of_block_names.iter().any(|name| s == name) => true,
            _ => false,
        }
    }
}

#[derive(Default)]
struct FnState {
    block_states: HashMap<Span, BlockState>,
}

impl FnState {
    fn expr_is_timestamp_variable(&self, expr: &Expr, blocks: &[Span]) -> bool {
        for block_span in blocks.iter().rev() {
            let block_state = self.block_states.get(block_span).unwrap();
            let Some(_) = block_state.var_states.iter().rev().find(|v| v.name == expr.span().as_str() && v.is_timestamp) else { continue };
            return true;
        }

        false
    }
}

#[derive(Default)]
struct BlockState {
    var_states: Vec<VarState>,
}

#[derive(Default)]
struct VarState {
    name: String,
    is_timestamp: bool,
}

impl AstVisitor for WeakPrngVisitor {
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

        // Check the use tree for `std::block::timestamp`
        if let Some(name) = utils::use_tree_to_name(&context.item_use.tree, "std::block::timestamp") {
            module_state.timestamp_names.push(name);
        }
        
        // Check the use tree for `std::block::timestamp_of_block`
        if let Some(name) = utils::use_tree_to_name(&context.item_use.tree, "std::block::timestamp_of_block") {
            module_state.timestamp_of_block_names.push(name);
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
        // Only check single variable reassignment expression statements
        let Statement::Expr {
            expr: Expr::Reassignment {
                assignable: Assignable::Var(ident),
                expr,
                ..
            },
            ..
        } = context.statement else { return Ok(()) };

        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Check if the expression is a timestamp call
        let mut is_timestamp = module_state.expr_is_timestamp_call(expr);

        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get_mut(&fn_signature).unwrap();

        // Check if variable is being reassigned to another variable that is a timestamp
        if fn_state.expr_is_timestamp_variable(expr, context.blocks.as_slice()) {
            is_timestamp = true;
        }

        // Get the block state
        let block_span = context.blocks.last().unwrap();
        let block_state = fn_state.block_states.get_mut(block_span).unwrap();

        // Find last variable
        let Some(var_state) = block_state.var_states.iter_mut().rev().find(|v| v.name == ident.as_str()) else { return Ok(()) };
        
        // Update the variable state
        var_state.is_timestamp = is_timestamp;
        
        Ok(())
    }

    fn visit_statement_let(&mut self, context: &StatementLetContext, _project: &mut Project) -> Result<(), Error> {
        // Only check single variable patterns
        let Pattern::AmbiguousSingleIdent(ident) = &context.statement_let.pattern else { return Ok(()) };

        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Check if the expression is a timestamp call
        let mut is_timestamp = module_state.expr_is_timestamp_call(&context.statement_let.expr);

        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get_mut(&fn_signature).unwrap();

        // Check if variable is being assigned to another variable that is a timestamp
        if fn_state.expr_is_timestamp_variable(&context.statement_let.expr, context.blocks.as_slice()) {
            is_timestamp = true;
        }
        
        // Get the block state
        let block_span = context.blocks.last().unwrap();
        let block_state = fn_state.block_states.get_mut(block_span).unwrap();

        // Create the variable state
        block_state.var_states.push(VarState {
            name: ident.to_string(),
            is_timestamp,
        });

        Ok(())
    }

    fn visit_expr(&mut self, context: &ExprContext, project: &mut Project) -> Result<(), Error> {
        // Only check modulo expressions
        let Expr::Modulo { lhs, .. } = context.expr else { return Ok(()) };

        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Check if `lhs` is a timestamp call expression
        let mut is_timestamp = module_state.expr_is_timestamp_call(lhs.as_ref());

        // Get the function state if available
        if let Some(item_fn) = context.item_fn.as_ref() {
            let fn_signature = item_fn.fn_signature.span();
            let fn_state = module_state.fn_states.get_mut(&fn_signature).unwrap();

            // Check if `lhs` is a timestamp variable expression
            if fn_state.expr_is_timestamp_variable(lhs.as_ref(), context.blocks.as_slice()) {
                is_timestamp = true;
            }
        }

        if is_timestamp {
            project.report.borrow_mut().add_entry(
                context.path,
                project.span_to_line(context.path, &context.expr.span())?,
                Severity::Medium,
                format!(
                    "{} contains weak PRNG due to dependence on a block timestamp: `{}`",
                    utils::get_item_location(context.item, &context.item_impl, &context.item_fn),
                    context.expr.span().as_str(),
                ),
            );
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_weak_prng() {
        crate::tests::test_detector("weak_prng", 18);
    }
}
