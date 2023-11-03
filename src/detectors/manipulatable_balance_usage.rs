use crate::{
    error::Error,
    project::Project,
    report::Severity,
    utils,
    visitor::{AstVisitor, FnContext, ModuleContext, StorageContext, UseContext},
};
use std::{collections::HashMap, path::PathBuf};
use sway_ast::{Expr, Statement, StatementLet};
use sway_types::{Span, Spanned};

#[derive(Default)]
pub struct ManipulatableBalanceUsageVisitor {
    module_states: HashMap<PathBuf, ModuleState>,
}

struct ModuleState {
    balances: HashMap<Span, String>,
    balances_used: HashMap<Span, String>,
    fn_calls_to_check: Vec<String>,
}

impl Default for ModuleState {
    fn default() -> Self {
        Self {
            balances: HashMap::new(),
            balances_used: HashMap::new(),
            fn_calls_to_check: vec![
                "std::token::transfer".into(),
                "std::token::transfer_to_address".into(),
                "std::token::force_transfer_to_contract".into(),
                "std::low_level_call::call_with_function_selector".into(),
            ],
        }
    }
}

impl AstVisitor for ManipulatableBalanceUsageVisitor {
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

        // Check the use tree for `std::token::transfer`
        if let Some(name) = utils::use_tree_to_name(&context.item_use.tree, "std::token::transfer") {
            module_state.fn_calls_to_check.push(name);
        }

        // Check the use tree for `std::token::transfer_to_address`
        if let Some(name) = utils::use_tree_to_name(&context.item_use.tree, "std::token::transfer_to_address") {
            module_state.fn_calls_to_check.push(name);
        }

        // Check the use tree for `std::token::force_transfer_to_contract`
        if let Some(name) = utils::use_tree_to_name(&context.item_use.tree, "std::token::force_transfer_to_contract") {
            module_state.fn_calls_to_check.push(name);
        }

        // Check the use tree for `std::low_level_call::call_with_function_selector`
        if let Some(name) = utils::use_tree_to_name(&context.item_use.tree, "std::low_level_call::call_with_function_selector") {
            module_state.fn_calls_to_check.push(name);
        }

        Ok(())
    }

    fn visit_storage(&mut self, context: &StorageContext, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        for field in &context.item_storage.fields.inner {
            if field.value.span().as_str().contains("balance") {
                module_state.balances.insert(field.value.span(), field.value.name.span().as_str().to_string());
            }
        }

        Ok(())
    }

    fn visit_fn(&mut self, context: &FnContext, project: &mut Project) -> Result<(), Error> {
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // First get the storage access points for balances in the function body
        for statement in &context.item_fn.body.inner.statements {
            if module_state.balances.iter().any(|(_, s)| statement.span().as_str().contains(format!("storage.{}", s).as_str())) {
                module_state.balances_used.insert(statement.span(), statement.span().as_str().to_string());
            }
        }

        // Then check if we are assigning a balance to a variable
        for statement in &context.item_fn.body.inner.statements {
            let Statement::Let(StatementLet { expr, .. }) = statement else { continue };

            let (Expr::Div { lhs, rhs, .. }
            | Expr::Mul { lhs, rhs, .. }
            | Expr::Add { lhs, rhs, .. }
            | Expr::Sub { lhs, rhs, .. }) = expr else { continue };

            if module_state.balances_used.iter().any(|(sp, _)| sp.as_str().contains(lhs.span().as_str()))
            || module_state.balances_used.iter().any(|(sp, _)| sp.as_str().contains(rhs.span().as_str())) {
                module_state.balances_used.insert(statement.span(), statement.span().as_str().to_string());
            }
        }
        
        // The check if there are any transfers in the function body that use the balances
        for statement in &context.item_fn.body.inner.statements {
            let sway_ast::Statement::Expr { expr, .. } = statement else { continue };
            let sway_ast::Expr::FuncApp { func, args } = expr else { continue };

            if module_state.fn_calls_to_check.iter().any(|x| x.contains(&func.span().as_str())) {
                let final_arg = if let Some(final_arg) = args.inner.final_value_opt.as_ref() {
                    final_arg.as_ref()
                } else if let Some(arg) = args.inner.value_separator_pairs.last() {
                    &arg.0
                } else {
                    continue
                };

                if module_state.balances_used.iter().any(|(_, st)| st.contains(final_arg.span().as_str())) {
                    project.report.borrow_mut().add_entry(
                        context.path,
                        project.span_to_line(context.path, &expr.span())?,
                        Severity::Medium,
                        format!(
                            "{} contains manipulatable balance usage: `{}`",
                            utils::get_item_location(context.item, &context.item_impl, &Some(context.item_fn)),
                            expr.span().as_str(),
                        ),
                    );
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_manipulatable_balance_usage() {
        crate::tests::test_detector("manipulatable_balance_usage", 5);
    }
}
