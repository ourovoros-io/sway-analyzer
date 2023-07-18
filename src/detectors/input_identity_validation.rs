use std::{collections::HashMap, path::PathBuf};

use sway_types::{Span, Spanned};

use crate::visitor::AstVisitor;

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
    fn visit_module(
        &mut self,
        context: &crate::visitor::ModuleContext,
        _project: &mut crate::project::Project,
    ) -> Result<(), crate::error::Error> {
        // Create the module state
        if !self.module_states.contains_key(context.path) {
            self.module_states
                .insert(context.path.into(), ModuleState::default());
        }

        Ok(())
    }

    fn visit_fn(
        &mut self,
        context: &crate::visitor::FnContext,
        _project: &mut crate::project::Project,
    ) -> Result<(), crate::error::Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Create the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.entry(fn_signature).or_insert(FnState::default());

        match &context.item_fn.fn_signature.arguments.inner {
            sway_ast::FnArgs::Static(args) => {
                for arg in args.value_separator_pairs.iter() {
                    match arg.0.ty.span().as_str() {
                        "Identity" => {
                            fn_state.identity_checks.insert(arg.0.pattern.span(), false);
                        }

                        "ContractId" => {
                            fn_state.contract_id_checks.insert(arg.0.pattern.span(), false);
                        }

                        "Address" => {
                            fn_state.address_checks.insert(arg.0.pattern.span(), false);
                        }

                        _ => {}
                    }
                }

                if let Some(arg) = args.final_value_opt.as_ref() {
                    match arg.ty.span().as_str() {
                        "Identity" => {
                            fn_state.identity_checks.insert(arg.pattern.span(), false);
                        }

                        "ContractId" => {
                            fn_state.contract_id_checks.insert(arg.pattern.span(), false);
                        }

                        "Address" => {
                            fn_state.address_checks.insert(arg.pattern.span(), false);
                        }

                        _ => {}
                    }
                }
            }
            
            sway_ast::FnArgs::NonStatic { self_token, ref_self, mutable_self, args_opt } => todo!(),
        }

        Ok(())
    }

    fn leave_fn(
        &mut self,
        context: &crate::visitor::FnContext,
        project: &mut crate::project::Project,
    ) -> Result<(), crate::error::Error> {
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

    fn visit_block(
        &mut self,
        context: &crate::visitor::BlockContext,
        _project: &mut crate::project::Project,
    ) -> Result<(), crate::error::Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get_mut(&fn_signature).unwrap();

        // Create the block state
        let block_span = context.block.span();

        if !fn_state.block_states.contains_key(&block_span) {
            fn_state
                .block_states
                .insert(block_span, BlockState::default());
        }

        Ok(())
    }

    fn visit_match_expr(
        &mut self,
        context: &crate::visitor::MatchExprContext,
        project: &mut crate::project::Project,
    ) -> Result<(), crate::error::Error> {
        println!("{:#?}", context.expr);
        Ok(())
    }
}
