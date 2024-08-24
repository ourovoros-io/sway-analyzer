use crate::{
    error::Error,
    project::Project,
    report::Severity,
    scope::AstScope,
    utils,
    visitor::{AstVisitor, AstVisitorRecursive, ExprContext, FnContext, ModuleContext, UseContext},
};
use std::{cell::RefCell, collections::HashMap, path::PathBuf, rc::Rc};
use sway_ast::{Expr, FnArgs, IfCondition, PathType, Ty};
use sway_types::{Span, Spanned};

#[derive(Default)]
pub struct ArbitraryAssetTransferVisitor {
    module_states: HashMap<PathBuf, ModuleState>,
}

struct ModuleState {
    fn_states: HashMap<Span, FnState>,
    fn_calls_to_check: Vec<String>,
    storage_accounts: Vec<String>,
}

impl Default for ModuleState {
    fn default() -> Self {
        Self {
            fn_states: HashMap::new(),
            fn_calls_to_check: vec![
                "std::asset::transfer".into(),
                "std::asset::transfer_to_address".into(),
                "std::asset::force_transfer_to_contract".into(),
                "std::low_level_call::call_with_function_selector".into(),
            ],
            storage_accounts: Vec::new(),
        }
    }
}

#[derive(Default, Debug)]
struct FnState {
    has_amount: bool,
    has_identity: bool,
    has_requirement: bool,
}

impl AstVisitor for ArbitraryAssetTransferVisitor {
    fn visit_module(&mut self, context: &ModuleContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        // Create the module state
        if !self.module_states.contains_key(context.path) {
            self.module_states.insert(context.path.into(), ModuleState::default());
        }

        // Collect storage information ahead of time
        let mut preprocess_visitor = AstVisitorRecursive::default();

        preprocess_visitor.visit_storage_field_hooks.push(Box::new(|context, _scope, _project| {
            // Get the module state
            let module_state = self.module_states.get_mut(context.path).unwrap();

            if context.field.ty.span().str().contains("Identity") {
                module_state.storage_accounts.push(context.field.name.span().str());
            }

            Ok(())
        }));

        preprocess_visitor.visit_module(context, scope.clone(), project)?;
        preprocess_visitor.leave_module(context, scope.clone(), project)?;

        Ok(())
    }

    fn visit_use(&mut self, context: &UseContext, _scope: Rc<RefCell<AstScope>>, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Check the use tree for `std::asset::transfer`
        if let Some(name) = utils::use_tree_to_name(&context.item_use.tree, "std::asset::transfer") {
            module_state.fn_calls_to_check.push(name);
        }
        
        // Check the use tree for `std::asset::transfer_to_address`
        if let Some(name) = utils::use_tree_to_name(&context.item_use.tree, "std::asset::transfer_to_address") {
            module_state.fn_calls_to_check.push(name);
        }

        // Check the use tree for `std::asset::force_transfer_to_contract`
        if let Some(name) = utils::use_tree_to_name(&context.item_use.tree, "std::asset::force_transfer_to_contract") {
            module_state.fn_calls_to_check.push(name);
        }

        // Check the use tree for `std::low_level_call::call_with_function_selector`
        if let Some(name) = utils::use_tree_to_name(&context.item_use.tree, "std::low_level_call::call_with_function_selector") {
            module_state.fn_calls_to_check.push(name);
        }

        Ok(())
    }

    fn visit_fn(&mut self, context: &FnContext, _scope: Rc<RefCell<AstScope>>, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Create the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.entry(fn_signature.clone()).or_default();

        // Check to see if the function contains `amount` or `identity` arguments
        let args = match &context.item_fn.fn_signature.arguments.inner {
            FnArgs::Static(args) => args,
            FnArgs::NonStatic { args_opt: Some(args), .. } => &args.1,
            _ => return Ok(()),
        };

        for arg in args {
            if let Ty::Path(PathType { prefix, .. }) = &arg.ty {
                match prefix.span().as_str() {
                    "u64" => fn_state.has_amount = true,
                    "Identity" => fn_state.has_identity = true,
                    _ => {}
                }
            }
        }

        Ok(())
    }

    fn visit_expr(&mut self, context: &ExprContext, _scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Get the function state
        let Some(item_fn) = context.item_fn else { return Ok(()) };
        let fn_signature = item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get_mut(&fn_signature).unwrap();

        if fn_state.has_requirement {
            return Ok(());
        }
    
        // Check for `require` and update the function state
        if utils::get_require_args(context.expr).is_some() {
            if module_state.storage_accounts.iter().any(|x| context.expr.span().as_str().contains(x)) {
                fn_state.has_requirement = true;
            }
        }
        // Check for `if/revert` and update the function state
        else if let Some(IfCondition::Expr(expr)) = utils::get_if_revert_condition(context.expr) {
            if module_state.storage_accounts.iter().any(|x| expr.span().as_str().contains(x)) {
                fn_state.has_requirement = true;
            }
        }
        // Check for calls to `transfer` functions
        else if let Expr::FuncApp { func, args } = context.expr {
            let Some(_) = module_state.fn_calls_to_check.iter().find(|&x| x == func.span().as_str()) else { return Ok(()) };
        
            if fn_state.has_amount && fn_state.has_identity || module_state.storage_accounts.iter().any(|acc| args.span().as_str().contains(acc) && acc != "admin") {
                project.report.borrow_mut().add_entry(
                    context.path,
                    project.span_to_line(context.path, &context.expr.span())?,
                    Severity::High,
                    format!(
                        "{} contains an arbitrary native asset transfer: `{}`",
                        utils::get_item_location(context.item, &context.item_impl, &context.item_fn),
                        context.expr.span().as_str(),
                    ),
                );
            } 
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_arbitrary_asset_transfer() {
        crate::tests::test_detector("arbitrary_asset_transfer", 8);
    }
}
