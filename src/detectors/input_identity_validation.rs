use crate::{
    error::Error,
    project::Project,
    visitor::{AstVisitor, BlockContext, FnContext, MatchExprContext, ModuleContext},
};
use std::{collections::HashMap, path::PathBuf};
use sway_ast::{FnArg, FnArgs};
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
    // TODO
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

    fn visit_match_expr(&mut self, context: &MatchExprContext, _project: &mut Project) -> Result<(), Error> {
        println!("{:#?}", context.expr);
        Ok(())
    }
}
