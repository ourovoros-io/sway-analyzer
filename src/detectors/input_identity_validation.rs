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
    address_checks: HashMap<Span, bool>,
    contract_id_checks: HashMap<Span, bool>,
    identity_checks: HashMap<Span, (bool, bool)>,
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
                    fn_state.identity_checks.insert(arg.pattern.span(), (false, false));
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

        let create_report_entry = |parameter_span: &Span| -> Result<(), Error> {
            project.report.borrow_mut().add_entry(
                context.path,
                project.span_to_line(context.path, parameter_span)?,
                format!(
                    "The `{}` function does not check its `{}` parameter for a zero value.",
                    if let Some(item_impl) = context.item_impl.as_ref() {
                        format!(
                            "{}::{}",
                            item_impl.ty.span().as_str(),
                            context.item_fn.fn_signature.name.as_str(),
                        )
                    } else {
                        format!(
                            "{}",
                            context.item_fn.fn_signature.name.as_str(),
                        )
                    },
                    parameter_span.as_str(),
                ),
            );

            Ok(())
        };

        // Check for any unchecked parameters of type `Address`
        for (parameter_span, address_checked) in fn_state.address_checks.iter() {
            if !address_checked {
                create_report_entry(parameter_span)?;
            }
        }

        // Check for any unchecked parameters of type `ContractId`
        for (parameter_span, contract_id_checked) in fn_state.contract_id_checks.iter() {
            if !contract_id_checked {
                create_report_entry(parameter_span)?;
            }
        }

        // Check for any unchecked parameters of type `Identity`
        for (parameter_span, (address_checked, contract_id_checked)) in fn_state.identity_checks.iter() {
            if !address_checked || !contract_id_checked {
                create_report_entry(parameter_span)?;
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
        let Some(block_span) = context.blocks.last() else { return Ok(()) };
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
            return Ok(());
        };

        match expr {
            Expr::Match { value, branches, .. } => {
                //
                // Check for the following pattern:
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
                if let Some((_, (address_checked, contract_id_checked))) = fn_state.identity_checks.iter_mut().find(|(x, _)| x.as_str() == value.span().as_str()) {
                    //
                    // Check `branches` for `Identity::Address` and `Identity::ContractId` zero value checks
                    //

                    for branch in branches.inner.iter() {
                        let Pattern::Constructor { path, args } = &branch.pattern else { continue };
                        let "Identity" = path.prefix.name.as_str() else { continue };
                        let Some(suffix) = path.suffix.last() else { continue };
                        
                        let mut ident = None;

                        match args.inner.final_value_opt.as_ref() {
                            Some(arg) => match arg.as_ref() {
                                Pattern::AmbiguousSingleIdent(arg) => ident = Some(arg),
                                _ => {}
                            }

                            None => match args.inner.value_separator_pairs.first() {
                                Some(arg) => match &arg.0 {
                                    Pattern::AmbiguousSingleIdent(arg) => ident = Some(arg),
                                    _ => {}
                                }

                                None => {}
                            }
                        }

                        let Some(ident) = ident else { continue };

                        let mut check_expr = |expr: &Expr| {
                            let Expr::FuncApp { func, args } = expr else { return };
                            let "require" = func.span().as_str() else { return };

                            let input = match args.inner.value_separator_pairs.first() {
                                Some(arg) => &arg.0,
                                None => match args.inner.final_value_opt.as_ref() {
                                    Some(arg) => arg.as_ref(),
                                    None => return,
                                }
                            };

                            let Expr::NotEqual { lhs, rhs, .. } = input else { return };

                            let zero_value = if lhs.span().as_str() == ident.span().as_str() {
                                rhs.as_ref()
                            } else if rhs.span().as_str() == ident.span().as_str() {
                                lhs.as_ref()
                            } else {
                                return;
                            };

                            let Expr::FuncApp { func, args } = zero_value else { return };
                            if func.span().as_str() != format!("{}::from", suffix.1.name.as_str()) { return; }
                            if args.span().as_str() != "(ZERO_B256)" { return; }

                            match suffix.1.name.as_str() {
                                "Address" => *address_checked = true,
                                "ContractId" => *contract_id_checked = true,
                                _ => {}
                            }
                        };

                        match &branch.kind {
                            sway_ast::MatchBranchKind::Block { block, .. } => {
                                for statement in block.inner.statements.iter() {
                                    let Statement::Expr { expr, .. } = statement else { continue };
                                    check_expr(expr);
                                }

                                if let Some(expr) = block.inner.final_expr_opt.as_ref() {
                                    check_expr(expr);
                                }
                            }

                            sway_ast::MatchBranchKind::Expr { expr, .. } => {
                                check_expr(expr);
                            }
                        }
                    }
                }
            }
            
            Expr::If(if_expr) => {
                //
                // Check for the following pattern:
                //
                // if let Identity::Address(x) = to {
                //     require(x != Address::from(ZERO_B256), "Zero address");
                // } else if let Identity::ContractId(x) = to {
                //     require(x != ContractId::from(ZERO_B256), "Zero contract id");
                // }
                //

                let mut next_if_expr = Some(if_expr);

                'if_expr_check: while let Some(if_expr) = next_if_expr {
                    let IfExpr {
                        condition: IfCondition::Let { lhs, rhs, .. },
                        then_block,
                        else_opt,
                        ..
                    } = if_expr else {
                        break;
                    };

                    // Check if `rhs` is a variable declaration, skip if so
                    for block_span in context.blocks.iter().rev() {
                        let block_state = fn_state.block_states.get(block_span).unwrap();
    
                        if block_state.variables.iter().any(|v| v.as_str() == rhs.span().as_str()) {
                            // Jump to the next if expression if available
                            if let Some((_, LoopControlFlow::Continue(else_if_expr))) = else_opt {
                                next_if_expr = Some(else_if_expr.as_ref());
                            } else {
                                next_if_expr = None;
                            }

                            continue 'if_expr_check;
                        }
                    }
    
                    // Check if `rhs` is a parameter of type `Identity`
                    if let Some((_, (address_checked, contract_id_checked))) = fn_state.identity_checks.iter_mut().find(|(x, _)| x.as_str() == rhs.span().as_str()) {
                        //
                        // Check `then_block` statements for `Identity::Address` and `Identity::ContractId` zero value checks
                        //

                        let Pattern::Constructor { path, args } = lhs.as_ref() else { continue };
                        let "Identity" = path.prefix.name.as_str() else { continue };
                        let Some(suffix) = path.suffix.last() else { continue };
                        
                        let mut ident = None;

                        match args.inner.final_value_opt.as_ref() {
                            Some(arg) => match arg.as_ref() {
                                Pattern::AmbiguousSingleIdent(arg) => ident = Some(arg),
                                _ => {}
                            }

                            None => match args.inner.value_separator_pairs.first() {
                                Some(arg) => match &arg.0 {
                                    Pattern::AmbiguousSingleIdent(arg) => ident = Some(arg),
                                    _ => {}
                                }

                                None => {}
                            }
                        }

                        let Some(ident) = ident else { continue };

                        let mut check_expr = |expr: &Expr| {
                            let Expr::FuncApp { func, args } = expr else { return };
                            let "require" = func.span().as_str() else { return };

                            let input = match args.inner.value_separator_pairs.first() {
                                Some(arg) => &arg.0,
                                None => match args.inner.final_value_opt.as_ref() {
                                    Some(arg) => arg.as_ref(),
                                    None => return,
                                }
                            };

                            let Expr::NotEqual { lhs, rhs, .. } = input else { return };

                            let zero_value = if lhs.span().as_str() == ident.span().as_str() {
                                rhs.as_ref()
                            } else if rhs.span().as_str() == ident.span().as_str() {
                                lhs.as_ref()
                            } else {
                                return;
                            };

                            let Expr::FuncApp { func, args } = zero_value else { return };
                            if func.span().as_str() != format!("{}::from", suffix.1.name.as_str()) { return; }
                            if args.span().as_str() != "(ZERO_B256)" { return; }

                            match suffix.1.name.as_str() {
                                "Address" => *address_checked = true,
                                "ContractId" => *contract_id_checked = true,
                                _ => {}
                            }
                        };

                        for statement in then_block.inner.statements.iter() {
                            let Statement::Expr { expr, .. } = statement else { continue };
                            check_expr(expr);
                        }

                        if let Some(expr) = then_block.inner.final_expr_opt.as_ref() {
                            check_expr(expr);
                        }
                    }

                    // Jump to the next if expression if available
                    if let Some((_, LoopControlFlow::Continue(else_if_expr))) = else_opt {
                        next_if_expr = Some(else_if_expr.as_ref());
                    } else {
                        next_if_expr = None;
                    }
                }
            }
            
            Expr::FuncApp { func, args } => {
                // Only check require calls
                let "require" = func.span().as_str() else { return Ok(()) };

                let input = match args.inner.value_separator_pairs.first() {
                    Some(arg) => &arg.0,
                    None => match args.inner.final_value_opt.as_ref() {
                        Some(arg) => arg.as_ref(),
                        None => return Ok(()),
                    }
                };

                match input {
                    Expr::NotEqual { lhs, rhs, .. } => {
                        //
                        // Check for the following patterns:
                        //
                        // require(x != Address::from(ZERO_B256), "Zero address");
                        // require(x != ContractId::from(ZERO_B256), "Zero contract id");
                        //

                        // Check if `lhs` is a variable declaration, skip if so
                        for block_span in context.blocks.iter().rev() {
                            let block_state = fn_state.block_states.get(block_span).unwrap();
        
                            if block_state.variables.iter().any(|v| v.as_str() == lhs.span().as_str()) {
                                return Ok(());
                            }
                        }
                        
                        // Check if `lhs` is a parameter of type `Address`
                        if let Some((_, address_checked)) = fn_state.address_checks.iter_mut().find(|(x, _)| x.as_str() == lhs.span().as_str()) {
                            let Expr::FuncApp { func, args } = rhs.as_ref() else { return Ok(()) };
                            let "Address::from" = func.span().as_str() else { return Ok(()) };
                            let "(ZERO_B256)" = args.span().as_str() else { return Ok(()) };
                            *address_checked = true;
                        }
                        // Check if `lhs` is a parameter of type `ContractId`
                        else if let Some((_, contract_id_checked)) = fn_state.contract_id_checks.iter_mut().find(|(x, _)| x.as_str() == lhs.span().as_str()) {
                            let Expr::FuncApp { func, args } = rhs.as_ref() else { return Ok(()) };
                            let "ContractId::from" = func.span().as_str() else { return Ok(()) };
                            let "(ZERO_B256)" = args.span().as_str() else { return Ok(()) };
                            *contract_id_checked = true;
                        }
                    }

                    Expr::Match { value, branches, .. } => {
                        //
                        // Check for the following pattern:
                        //
                        // match to {
                        //     Identity::Address(x) => x != Address::from(ZERO_B256),
                        //     Identity::ContractId(x) => x != ContractId::from(ZERO_B256),
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
                        if let Some((_, (address_checked, contract_id_checked))) = fn_state.identity_checks.iter_mut().find(|(x, _)| x.as_str() == value.span().as_str()) {
                            //
                            // Check `branches` for `Identity::Address` and `Identity::ContractId` zero value checks
                            //

                            for branch in branches.inner.iter() {
                                let Pattern::Constructor { path, args } = &branch.pattern else { continue };
                                let "Identity" = path.prefix.name.as_str() else { continue };
                                let Some(suffix) = path.suffix.last() else { continue };
                                
                                let mut ident = None;

                                match args.inner.final_value_opt.as_ref() {
                                    Some(arg) => match arg.as_ref() {
                                        Pattern::AmbiguousSingleIdent(arg) => ident = Some(arg),
                                        _ => {}
                                    }

                                    None => match args.inner.value_separator_pairs.first() {
                                        Some(arg) => match &arg.0 {
                                            Pattern::AmbiguousSingleIdent(arg) => ident = Some(arg),
                                            _ => {}
                                        }

                                        None => {}
                                    }
                                }

                                let Some(ident) = ident else { continue };

                                let mut check_expr = |expr: &Expr| {
                                    let Expr::NotEqual { lhs, rhs, .. } = expr else { return };

                                    let zero_value = if lhs.span().as_str() == ident.span().as_str() {
                                        rhs.as_ref()
                                    } else if rhs.span().as_str() == ident.span().as_str() {
                                        lhs.as_ref()
                                    } else {
                                        return;
                                    };

                                    let Expr::FuncApp { func, args } = zero_value else { return };
                                    if func.span().as_str() != format!("{}::from", suffix.1.name.as_str()) { return; }
                                    if args.span().as_str() != "(ZERO_B256)" { return; }

                                    match suffix.1.name.as_str() {
                                        "Address" => *address_checked = true,
                                        "ContractId" => *contract_id_checked = true,
                                        _ => {}
                                    }
                                };

                                match &branch.kind {
                                    sway_ast::MatchBranchKind::Block { block, .. } => {
                                        for statement in block.inner.statements.iter() {
                                            let Statement::Expr { expr, .. } = statement else { continue };
                                            check_expr(expr);
                                        }

                                        if let Some(expr) = block.inner.final_expr_opt.as_ref() {
                                            check_expr(expr);
                                        }
                                    }

                                    sway_ast::MatchBranchKind::Expr { expr, .. } => {
                                        check_expr(expr);
                                    }
                                }
                            }
                        }
                    }

                    Expr::If(if_expr) => {
                        //
                        // Check for the following pattern:
                        //
                        // if let Identity::Address(x) = to {
                        //     x != Address::from(ZERO_B256)
                        // } else if let Identity::ContractId(x) = to {
                        //     x != ContractId::from(ZERO_B256)
                        // }
                        //

                        let mut next_if_expr = Some(if_expr);

                        'if_expr_check: while let Some(if_expr) = next_if_expr {
                            let IfExpr {
                                condition: IfCondition::Let { lhs, rhs, .. },
                                then_block,
                                else_opt,
                                ..
                            } = if_expr else {
                                break;
                            };

                            // Check if `rhs` is a variable declaration, skip if so
                            for block_span in context.blocks.iter().rev() {
                                let block_state = fn_state.block_states.get(block_span).unwrap();
            
                                if block_state.variables.iter().any(|v| v.as_str() == rhs.span().as_str()) {
                                    // Jump to the next if expression if available
                                    if let Some((_, LoopControlFlow::Continue(else_if_expr))) = else_opt {
                                        next_if_expr = Some(else_if_expr.as_ref());
                                    } else {
                                        next_if_expr = None;
                                    }

                                    continue 'if_expr_check;
                                }
                            }
            
                            // Check if `rhs` is a parameter of type `Identity`
                            if let Some((_, (address_checked, contract_id_checked))) = fn_state.identity_checks.iter_mut().find(|(x, _)| x.as_str() == rhs.span().as_str()) {
                                //
                                // Check `then_block` statements for `Identity::Address` and `Identity::ContractId` zero value checks
                                //

                                let Pattern::Constructor { path, args } = lhs.as_ref() else { continue };
                                let "Identity" = path.prefix.name.as_str() else { continue };
                                let Some(suffix) = path.suffix.last() else { continue };
                                
                                let mut ident = None;

                                match args.inner.final_value_opt.as_ref() {
                                    Some(arg) => match arg.as_ref() {
                                        Pattern::AmbiguousSingleIdent(arg) => ident = Some(arg),
                                        _ => {}
                                    }

                                    None => match args.inner.value_separator_pairs.first() {
                                        Some(arg) => match &arg.0 {
                                            Pattern::AmbiguousSingleIdent(arg) => ident = Some(arg),
                                            _ => {}
                                        }

                                        None => {}
                                    }
                                }

                                let Some(ident) = ident else { continue };

                                let mut check_expr = |expr: &Expr| {
                                    let Expr::NotEqual { lhs, rhs, .. } = expr else { return };

                                    let zero_value = if lhs.span().as_str() == ident.span().as_str() {
                                        rhs.as_ref()
                                    } else if rhs.span().as_str() == ident.span().as_str() {
                                        lhs.as_ref()
                                    } else {
                                        return;
                                    };

                                    let Expr::FuncApp { func, args } = zero_value else { return };
                                    if func.span().as_str() != format!("{}::from", suffix.1.name.as_str()) { return; }
                                    if args.span().as_str() != "(ZERO_B256)" { return; }

                                    match suffix.1.name.as_str() {
                                        "Address" => *address_checked = true,
                                        "ContractId" => *contract_id_checked = true,
                                        _ => {}
                                    }
                                };

                                for statement in then_block.inner.statements.iter() {
                                    let Statement::Expr { expr, .. } = statement else { continue };
                                    check_expr(expr);
                                }

                                if let Some(expr) = then_block.inner.final_expr_opt.as_ref() {
                                    check_expr(expr);
                                }
                            }

                            // Jump to the next if expression if available
                            if let Some((_, LoopControlFlow::Continue(else_if_expr))) = else_opt {
                                next_if_expr = Some(else_if_expr.as_ref());
                            } else {
                                next_if_expr = None;
                            }
                        }
                    }

                    _ => {}
                }  
            }

            _ => {}
        }

        Ok(())
    }
}
