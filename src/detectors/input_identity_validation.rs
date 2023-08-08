use crate::{
    error::Error,
    project::Project,
    visitor::{AstVisitor, BlockContext, FnContext, ModuleContext, StatementContext},
};
use std::{collections::HashMap, path::PathBuf};
use sway_ast::{
    expr::LoopControlFlow, Expr, FnArg, FnArgs, IfCondition, IfExpr, Pattern, Statement,
};
use sway_types::{Span, Spanned};

#[derive(Default)]
pub struct InputIdentityValidationVisitor {
    module_states: HashMap<PathBuf, ModuleState>,
}

#[derive(Default)]
struct ModuleState {
    fn_states: HashMap<Span, FnState>,
}

#[derive(Default)]
struct FnState {
    block_states: HashMap<Span, BlockState>,
    identity_checks: HashMap<Span, bool>,
    contract_id_checks: HashMap<Span, bool>,
    address_checks: HashMap<Span, bool>,
}

#[derive(Default)]
struct BlockState {
    variables: Vec<Span>,
}

impl AstVisitor for InputIdentityValidationVisitor {
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
        let fn_state = module_state.fn_states.entry(fn_signature).or_insert(FnState::default());

        // Check function arguments for `Address`, `ContractId` or `Identity` types and queue them to be checked
        let mut check_for_identity_argument = |arg: &FnArg| {
            match arg.ty.span().as_str() {
                "Address" => {
                    fn_state.address_checks.insert(arg.pattern.span(), false);
                }

                "ContractId" => {
                    fn_state.contract_id_checks.insert(arg.pattern.span(), false);
                }

                "Identity" => {
                    fn_state.identity_checks.insert(arg.pattern.span(), false);
                }

                _ => {}
            }
        };

        match &context.item_fn.fn_signature.arguments.inner {
            FnArgs::Static(args) => {
                for arg in args.value_separator_pairs.iter() {
                    check_for_identity_argument(&arg.0);
                }

                if let Some(arg) = args.final_value_opt.as_ref() {
                    check_for_identity_argument(arg);
                }
            }
            
            FnArgs::NonStatic { args_opt: Some(args), .. } => {
                for arg in args.1.value_separator_pairs.iter() {
                    check_for_identity_argument(&arg.0);
                }

                if let Some(arg) = args.1.final_value_opt.as_ref() {
                    check_for_identity_argument(arg);
                }
            }

            _ => {}
        }

        Ok(())
    }

    fn leave_fn(&mut self, context: &FnContext, project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get(context.path).unwrap();

        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get(&fn_signature).unwrap();

        // Check for any unchecked parameters
        for (parameter_span, parameter_checked) in fn_state.identity_checks.iter() {
            if !parameter_checked {
                project.report.borrow_mut().add_entry(
                    context.path,
                    project.span_to_line(context.path, parameter_span)?,
                    format!("Parameter `{}` is not checked for a zero value", parameter_span.as_str()),
                );
            }
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
        let block_state = fn_state.block_states.get_mut(&block_span).unwrap();

        // Store variable bindings declared in the current block in order to check if they shadow a parameter
        if let Statement::Let(item_let) = context.statement {
            match &item_let.pattern {
                //
                // TODO: handle other patterns
                //

                Pattern::Var { name, .. } => {
                    block_state.variables.push(name.span());
                }

                _ => {}
            }

            // Skip expression check since we know this is a variable binding
            return Ok(());
        }

        // Only check expression statements
        let Statement::Expr { expr, .. } = context.statement else {
            return Ok(())
        };

        match expr {
            Expr::Match { value, branches, .. } => {
                //
                // TODO: check for the following pattern
                //
                // match to {
                //     Identity::Address(x) => require(x != Address::from(ZERO_B256), "Zero address"),
                //     Identity::ContractId(x) => require(x != ContractId::from(ZERO_B256), "Zero contract id"),
                // }
                //

                // Check if `value` is a variable declaration, skip if so
                for block_span in context.blocks.iter().rev() {
                    let block_state = fn_state.block_states.get(block_span).unwrap();

                    if block_state.variables.iter().any(|v| v.as_str() == value.span().as_str()) {
                        return Ok(());
                    }
                }

                // Check if `value` is a parameter of type `Identity`
                if let Some((_, identity_checked)) = fn_state.identity_checks.iter_mut().find(|(x, _)| x.as_str() == value.span().as_str()) {
                    //
                    // TODO: check `branches` for `Identity::Address` and `Identity::ContractId` zero value checks
                    //
                }
            }
            
            Expr::If(if_expr) => {
                //
                // TODO: check for the following pattern
                //
                // if let Identity::Address(x) = to {
                //     require(x != Address::from(ZERO_B256), "Zero address");
                // } else if let Identity::ContractId(x) = to {
                //     require(x != ContractId::from(ZERO_B256), "Zero contract id");
                // }
                //

                let IfExpr {
                    condition: IfCondition::Let {
                        lhs: if_let_condition_lhs,
                        rhs: if_let_condition_rhs,
                        ..
                    },
                    then_block: if_let_then_block,
                    else_opt: Some((_, LoopControlFlow::Continue(else_if_expr))),
                    ..
                } = if_expr else {
                    return Ok(());
                };

                // Check if `if_let_condition_rhs` is a variable declaration, skip if so
                for block_span in context.blocks.iter().rev() {
                    let block_state = fn_state.block_states.get(block_span).unwrap();

                    if block_state.variables.iter().any(|v| v.as_str() == if_let_condition_rhs.span().as_str()) {
                        return Ok(());
                    }
                }

                // Check if `if_let_condition_rhs` is a parameter of type `Identity`
                if let Some((_, identity_checked)) = fn_state.identity_checks.iter_mut().find(|(x, _)| x.as_str() == if_let_condition_rhs.span().as_str()) {
                    //
                    // TODO: check `if_let_then_block` statements for `Identity::Address` and `Identity::ContractId` zero value checks
                    //
                }
                
                let IfExpr {
                    condition: IfCondition::Let {
                        lhs: else_if_let_condition_lhs,
                        rhs: else_if_let_condition_rhs,
                        ..
                    },
                    then_block: else_if_let_then_block,
                    ..
                } = else_if_expr.as_ref() else {
                    return Ok(());
                };

                // Check if `else_if_let_condition_rhs` is a variable declaration, skip if so
                for block_span in context.blocks.iter().rev() {
                    let block_state = fn_state.block_states.get(block_span).unwrap();

                    if block_state.variables.iter().any(|v| v.as_str() == else_if_let_condition_rhs.span().as_str()) {
                        return Ok(());
                    }
                }
                
                // Check if `else_if_let_condition_rhs` is a parameter of type `Identity`
                if let Some((_, identity_checked)) = fn_state.identity_checks.iter_mut().find(|(x, _)| x.as_str() == else_if_let_condition_rhs.span().as_str()) {
                    //
                    // TODO: check `else_if_let_then_block` statements for `Identity::Address` and `Identity::ContractId` zero value checks
                    //
                }
            }

            Expr::FuncApp { func, args } => {
                // Only check require calls
                if func.span().as_str() != "require" {
                    return Ok(());
                }

                //
                // TODO: check for the following pattern
                //
                // require(
                //     match to {
                //         Identity::Address(x) => x != Address::from(ZERO_B256),
                //         Identity::ContractId(x) => x != ContractId::from(ZERO_B256),
                //     },
                //     "Zero identity"
                // );
                //

                //
                // TODO: check for the following pattern
                //
                // require(
                //     if let Identity::Address(x) = to {
                //         x != Address::from(ZERO_B256)
                //     } else if let Identity::ContractId(x) = to {
                //         x != ContractId::from(ZERO_B256)
                //     } else {
                //         true
                //     },
                //     "Zero identity"
                // );
                //        
            }

            _ => {}
        }

        Ok(())
    }
}
