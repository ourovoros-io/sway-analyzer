use crate::{
    error::Error,
    project::Project,
    report::Severity,
    utils,
    visitor::{
        AstVisitor, BlockContext, FnContext, IfExprContext, ModuleContext, StatementContext,
    },
};
use std::{cell::RefCell, collections::HashMap, path::PathBuf, rc::Rc};
use sway_ast::{
    expr::LoopControlFlow, Expr, FnArg, FnArgs, IfCondition, IfExpr, MatchBranchKind, Statement,
};
use sway_types::{Span, Spanned};

#[derive(Default)]
pub struct NonZeroIdentityValidationVisitor {
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
    identity_checks: HashMap<Span, (Rc<RefCell<bool>>, Rc<RefCell<bool>>)>,
}

impl FnState {
    fn expr_is_variable(&self, expr: &Expr, blocks: &[Span]) -> bool {
        for block_span in blocks.iter().rev() {
            let block_state = self.block_states.get(block_span).unwrap();

            if block_state.variables.iter().any(|v| v.as_str() == expr.span().as_str()) {
                return true;
            }
        }

        false
    }

    fn find_address_check(&mut self, expr: &Expr) -> Option<&mut bool> {
        let span = expr.span();
        self.address_checks.iter_mut().find(|(x, _)| x.as_str() == span.as_str()).map(|x| x.1)
    }

    fn find_contract_id_check(&mut self, expr: &Expr) -> Option<&mut bool> {
        let span = expr.span();
        self.contract_id_checks.iter_mut().find(|(x, _)| x.as_str() == span.as_str()).map(|x| x.1)
    }

    fn find_identity_check(&mut self, expr: &Expr) -> Option<&mut (Rc<RefCell<bool>>, Rc<RefCell<bool>>)> {
        let span = expr.span();
        self.identity_checks.iter_mut().find(|(x, _)| x.as_str() == span.as_str()).map(|x| x.1)
    }

    fn apply_address_or_contract_id_check(&mut self, lhs: &Expr, rhs: &Expr) {
        // Check if `lhs` is a parameter of type `Address`
        if let Some(address_checked) = self.find_address_check(lhs) {
            if utils::is_zero_value_comparison("Address", lhs.span().as_str(), lhs, rhs) {
                *address_checked = true;
            }
        }
        // Check if `rhs` is a parameter of type `Address`
        else if let Some(address_checked) = self.find_address_check(rhs) {
            if utils::is_zero_value_comparison("Address", rhs.span().as_str(), lhs, rhs) {
                *address_checked = true;
            }
        }
        // Check if `lhs` is a parameter of type `ContractId`
        else if let Some(contract_id_checked) = self.find_contract_id_check(lhs) {
            if utils::is_zero_value_comparison("ContractId", lhs.span().as_str(), lhs, rhs) {
                *contract_id_checked = true;
            }
        }
        // Check if `rhs` is a parameter of type `ContractId`
        else if let Some(contract_id_checked) = self.find_contract_id_check(rhs) {
            if utils::is_zero_value_comparison("ContractId", rhs.span().as_str(), lhs, rhs) {
                *contract_id_checked = true;
            }
        }
    }
}

#[derive(Default)]
struct BlockState {
    variables: Vec<Span>,
}

impl AstVisitor for NonZeroIdentityValidationVisitor {
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
        let fn_state = module_state.fn_states.entry(fn_signature).or_insert_with(FnState::default);

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
                    fn_state.identity_checks.insert(arg.pattern.span(), (Rc::new(RefCell::new(false)), Rc::new(RefCell::new(false))));
                }

                _ => {}
            }
        };

        match &context.item_fn.fn_signature.arguments.inner {
            FnArgs::Static(args) => {
                for arg in args {
                    check_for_identity_argument(arg);
                }
            }
            
            FnArgs::NonStatic { args_opt: Some(args), .. } => {
                for arg in &args.1 {
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
                Severity::Low,
                format!(
                    "{} does not check its `{}` parameter for a zero value.",
                    utils::get_item_location(context.item, &context.item_impl, &Some(context.item_fn)),
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
            if !*address_checked.borrow() || !*contract_id_checked.borrow() {
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

    fn visit_if_expr(&mut self, context: &IfExprContext, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get_mut(&fn_signature).unwrap();

        // Check if the expression is an `if let`
        let IfCondition::Let { lhs, .. } = &context.if_expr.condition else { return Ok(()) };

        // Create the block state for the `if let` ahead of time
        let block_span = context.if_expr.then_block.span();
        let block_state = fn_state.block_states.entry(block_span).or_insert_with(BlockState::default);

        // Declare variable bindings from `lhs` inside the body block state of the `if let`
        for ident in utils::fold_pattern_idents(lhs.as_ref()) {
            block_state.variables.push(ident.span());
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

        // Store variable bindings declared in the current block in order to check if they shadow a parameter
        if let Statement::Let(item_let) = context.statement {
            for ident in utils::fold_pattern_idents(&item_let.pattern) {
                block_state.variables.push(ident.span());
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
                // Check for the following patterns:
                //
                // match to {
                //     Identity::Address(x) => require(x != Address::from(ZERO_B256), "Zero address"),
                //     Identity::ContractId(x) => require(x != ContractId::from(ZERO_B256), "Zero contract id"),
                // }
                //
                // match input {
                //     Identity::Address(x) => {
                //         if x == Address::from(ZERO_B256) {
                //             revert(0);
                //         }
                //     }
                //     Identity::ContractId(x) => {
                //         if x == ContractId::from(ZERO_B256) {
                //             revert(0);
                //         }
                //     }
                // }
                //

                // Check if `value` is a variable declaration, skip if so
                if fn_state.expr_is_variable(value.as_ref(), context.blocks.as_slice()) {
                    return Ok(());
                }

                // Check if `value` is a parameter of type `Identity`
                let Some((address_checked, contract_id_checked)) = fn_state.find_identity_check(value) else { return Ok(()) };

                // Check `branches` for `Identity::Address` and `Identity::ContractId` zero value checks
                for branch in branches.inner.iter() {
                    let Some((identity_kind, identity_value)) = utils::pattern_to_constructor_suffix_and_value("Identity", &branch.pattern) else { continue };

                    let check_for_require = |expr: &Expr| {
                        let Some(require_args) = utils::get_require_args(expr) else { return };
                        let Some(require_condition) = require_args.first() else { return };
                        let Expr::NotEqual { lhs, rhs, .. } = require_condition else { return };

                        if utils::is_zero_value_comparison(identity_kind.as_str(), identity_value.span().as_str(), lhs.as_ref(), rhs.as_ref()) {
                            match identity_kind.as_str() {
                                "Address" => *address_checked.borrow_mut() = true,
                                "ContractId" => *contract_id_checked.borrow_mut() = true,
                                _ => {}
                            }
                        }
                    };

                    let check_for_if_revert = |expr: &Expr| {
                        let Some(IfCondition::Expr(input)) = utils::get_if_revert_condition(expr) else { return };
                        let Expr::Equal { lhs, rhs, .. } = input.as_ref() else { return };

                        if utils::is_zero_value_comparison(identity_kind.as_str(), identity_value.span().as_str(), lhs.as_ref(), rhs.as_ref()) {
                            match identity_kind.as_str() {
                                "Address" => *address_checked.borrow_mut() = true,
                                "ContractId" => *contract_id_checked.borrow_mut() = true,
                                _ => {}
                            }
                        }
                    };

                    match &branch.kind {
                        MatchBranchKind::Block { block, .. } => {
                            for statement in block.inner.statements.iter() {
                                let Statement::Expr { expr, .. } = statement else { continue };
                                check_for_require(expr);
                                check_for_if_revert(expr);
                            }

                            if let Some(expr) = block.inner.final_expr_opt.as_ref() {
                                check_for_require(expr);
                                check_for_if_revert(expr);
                            }
                        }

                        MatchBranchKind::Expr { expr, .. } => {
                            check_for_require(expr);
                            check_for_if_revert(expr);
                        }
                    }
                }
            }
            
            Expr::If(if_expr) => match &if_expr.condition {
                IfCondition::Expr(expr) => match expr.as_ref() {
                    Expr::Equal { lhs, rhs, .. } => {
                        //
                        // Check for the following patterns:
                        //
                        // if input == Address::from(ZERO_B256) {
                        //     revert(0);
                        // }
                        //
                        // if input == ContractId::from(ZERO_B256) {
                        //     revert(0);
                        // }
                        //

                        // Check if `lhs` is a variable declaration, skip if so
                        if fn_state.expr_is_variable(lhs.as_ref(), context.blocks.as_slice()) {
                            return Ok(());
                        }

                        // Check if `if_expr.then_block` contains a revert
                        if !utils::block_has_revert(&if_expr.then_block) {
                            return Ok(());
                        }

                        // Check if `lhs` or `rhs` is a parameter of type `Address` or `ContractId`
                        fn_state.apply_address_or_contract_id_check(lhs.as_ref(), rhs.as_ref());
                    }

                    Expr::Match { value, branches, .. } => {
                        //
                        // Check for the following patterns:
                        //
                        // if match input {
                        //     Identity::Address(x) => x == Address::from(ZERO_B256),
                        //     Identity::ContractId(x) => x == ContractId::from(ZERO_B256),
                        // } {
                        //     revert(0);
                        // }
                        //

                        // Check if `value` is a variable declaration, skip if so
                        if fn_state.expr_is_variable(value.as_ref(), context.blocks.as_slice()) {
                            return Ok(());
                        }

                        // Check if `if_expr.then_block` contains a revert
                        if !utils::block_has_revert(&if_expr.then_block) {
                            return Ok(());
                        }

                        // Check if `value` is a parameter of type `Identity`
                        let Some((address_checked, contract_id_checked)) = fn_state.find_identity_check(value) else { return Ok(()) };
                    
                        // Check `branches` for `Identity::Address` and `Identity::ContractId` zero value checks
                        for branch in branches.inner.iter() {
                            let Some((identity_kind, identity_value)) = utils::pattern_to_constructor_suffix_and_value("Identity", &branch.pattern) else { continue };

                            let check_expr = |expr: &Expr| {
                                let Expr::Equal { lhs, rhs, .. } = expr else { return };

                                if utils::is_zero_value_comparison(identity_kind.as_str(), identity_value.span().as_str(), lhs.as_ref(), rhs.as_ref()) {
                                    match identity_kind.as_str() {
                                        "Address" => *address_checked.borrow_mut() = true,
                                        "ContractId" => *contract_id_checked.borrow_mut() = true,
                                        _ => {}
                                    }
                                }
                            };

                            match &branch.kind {
                                MatchBranchKind::Block { block, .. } => {
                                    for statement in block.inner.statements.iter() {
                                        let Statement::Expr { expr, .. } = statement else { continue };
                                        check_expr(expr);
                                    }

                                    if let Some(expr) = block.inner.final_expr_opt.as_ref() {
                                        check_expr(expr);
                                    }
                                }

                                MatchBranchKind::Expr { expr, .. } => {
                                    check_expr(expr);
                                }
                            }
                        }
                    }

                    Expr::If(first_if_expr) => {
                        //
                        // Check for the following patterns:
                        //
                        // if if let Identity::Address(x) = input {
                        //     x == Address::from(ZERO_B256)
                        // } else if let Identity::ContractId(x) = input {
                        //     x == ContractId::from(ZERO_B256)
                        // } else {
                        //     false
                        // } {
                        //     revert(0);
                        // }
                        //

                        // Check if `if_expr.then_block` contains a revert
                        if !utils::block_has_revert(&if_expr.then_block) {
                            return Ok(());
                        }

                        let mut next_if_expr = Some(first_if_expr);

                        while let Some(if_expr) = next_if_expr {
                            let IfExpr {
                                condition: IfCondition::Let { lhs, rhs, .. },
                                then_block,
                                else_opt,
                                ..
                            } = if_expr else {
                                break;
                            };

                            let mut check_if_expr = || {
                                // Check if `rhs` is a variable declaration, skip if so
                                if fn_state.expr_is_variable(rhs.as_ref(), context.blocks.as_slice()) {
                                    return;
                                }
                
                                // Check if `rhs` is a parameter of type `Identity`
                                let Some((address_checked, contract_id_checked)) = fn_state.find_identity_check(rhs.as_ref()) else { return };
    
                                // Check `then_block` statements for `Identity::Address` and `Identity::ContractId` zero value checks
                                let Some((identity_kind, identity_value)) = utils::pattern_to_constructor_suffix_and_value("Identity", lhs.as_ref()) else { return };
    
                                let check_expr = |expr: &Expr| {
                                    let Expr::Equal { lhs, rhs, .. } = expr else { return };
    
                                    if utils::is_zero_value_comparison(identity_kind.as_str(), identity_value.span().as_str(), lhs.as_ref(), rhs.as_ref()) {
                                        match identity_kind.as_str() {
                                            "Address" => *address_checked.borrow_mut() = true,
                                            "ContractId" => *contract_id_checked.borrow_mut() = true,
                                            _ => {}
                                        }
                                    }
                                };
    
                                for statement in then_block.inner.statements.iter() {
                                    let Statement::Expr { expr, .. } = statement else { continue };
                                    check_expr(expr);
                                }
    
                                if let Some(expr) = then_block.inner.final_expr_opt.as_ref() {
                                    check_expr(expr);
                                }
                            };

                            check_if_expr();
    
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
                
                IfCondition::Let { .. } => {
                    //
                    // Check for the following patterns:
                    //
                    // if let Identity::Address(x) = to {
                    //     require(x != Address::from(ZERO_B256), "Zero address");
                    // } else if let Identity::ContractId(x) = to {
                    //     require(x != ContractId::from(ZERO_B256), "Zero contract id");
                    // }
                    //
                    // if let Identity::Address(x) = input {
                    //     if x == Address::from(ZERO_B256) {
                    //         revert(0);
                    //     }
                    // } else if let Identity::ContractId(x) = input {
                    //     if x == ContractId::from(ZERO_B256) {
                    //         revert(0);
                    //     }
                    // }
                    //

                    let mut next_if_expr = Some(if_expr);

                    while let Some(if_expr) = next_if_expr {
                        let IfExpr {
                            condition: IfCondition::Let { lhs, rhs, .. },
                            then_block,
                            else_opt,
                            ..
                        } = if_expr else {
                            break;
                        };

                        let mut check_if_expr = || {
                            // Check if `rhs` is a variable declaration, skip if so
                            if fn_state.expr_is_variable(rhs.as_ref(), context.blocks.as_slice()) {
                                return;
                            }
            
                            // Check if `rhs` is a parameter of type `Identity`
                            let Some((address_checked, contract_id_checked)) = fn_state.find_identity_check(rhs.as_ref()) else { return };
                            
                            // Check `then_block` statements for `Identity::Address` and `Identity::ContractId` zero value checks
                            let Some((identity_kind, identity_value)) = utils::pattern_to_constructor_suffix_and_value("Identity", lhs.as_ref()) else { return };

                            let check_for_require = |expr: &Expr| {
                                let Some(require_args) = utils::get_require_args(expr) else { return };
                                let Some(require_condition) = require_args.first() else { return };
                                let Expr::NotEqual { lhs, rhs, .. } = require_condition else { return };

                                if utils::is_zero_value_comparison(identity_kind.as_str(), identity_value.span().as_str(), lhs.as_ref(), rhs.as_ref()) {
                                    match identity_kind.as_str() {
                                        "Address" => *address_checked.borrow_mut() = true,
                                        "ContractId" => *contract_id_checked.borrow_mut() = true,
                                        _ => {}
                                    }
                                }
                            };

                            let check_for_if_revert = |expr: &Expr| {
                                let Some(IfCondition::Expr(input)) = utils::get_if_revert_condition(expr) else { return };
                                let Expr::Equal { lhs, rhs, .. } = input.as_ref() else { return };

                                if utils::is_zero_value_comparison(identity_kind.as_str(), identity_value.span().as_str(), lhs.as_ref(), rhs.as_ref()) {
                                    match identity_kind.as_str() {
                                        "Address" => *address_checked.borrow_mut() = true,
                                        "ContractId" => *contract_id_checked.borrow_mut() = true,
                                        _ => {}
                                    }
                                }
                            };

                            for statement in then_block.inner.statements.iter() {
                                let Statement::Expr { expr, .. } = statement else { continue };
                                check_for_require(expr);
                                check_for_if_revert(expr);
                            }

                            if let Some(expr) = then_block.inner.final_expr_opt.as_ref() {
                                check_for_require(expr);
                                check_for_if_revert(expr);
                            }
                        };

                        check_if_expr();

                        // Jump to the next if expression if available
                        if let Some((_, LoopControlFlow::Continue(else_if_expr))) = else_opt {
                            next_if_expr = Some(else_if_expr.as_ref());
                        } else {
                            next_if_expr = None;
                        }
                    }
                }
            }
            
            Expr::FuncApp { .. } => {
                // Only check require calls
                let Some(require_args) = utils::get_require_args(expr) else { return Ok(()) };
                let Some(require_condition) = require_args.first() else { return Ok(()) };

                match require_condition {
                    Expr::NotEqual { lhs, rhs, .. } => {
                        //
                        // Check for the following patterns:
                        //
                        // x != Address::from(ZERO_B256)
                        // x != ContractId::from(ZERO_B256)
                        //

                        // Check if `lhs` is a variable declaration, skip if so
                        if fn_state.expr_is_variable(lhs.as_ref(), context.blocks.as_slice()) {
                            return Ok(());
                        }
                        
                        // Check if `lhs` or `rhs` is a parameter of type `Address` or `ContractId`
                        fn_state.apply_address_or_contract_id_check(lhs.as_ref(), rhs.as_ref());
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
                        if fn_state.expr_is_variable(value.as_ref(), context.blocks.as_slice()) {
                            return Ok(());
                        }

                        // Check if `value` is a parameter of type `Identity`
                        let Some((address_checked, contract_id_checked)) = fn_state.find_identity_check(value.as_ref()) else { return Ok(()) };
                            
                        // Check `branches` for `Identity::Address` and `Identity::ContractId` zero value checks
                        for branch in branches.inner.iter() {
                            let Some((identity_kind, identity_value)) = utils::pattern_to_constructor_suffix_and_value("Identity", &branch.pattern) else { continue };

                            let check_expr = |expr: &Expr| {
                                let Expr::NotEqual { lhs, rhs, .. } = expr else { return };

                                if utils::is_zero_value_comparison(identity_kind.as_str(), identity_value.span().as_str(), lhs.as_ref(), rhs.as_ref()) {
                                    match identity_kind.as_str() {
                                        "Address" => *address_checked.borrow_mut() = true,
                                        "ContractId" => *contract_id_checked.borrow_mut() = true,
                                        _ => {}
                                    }
                                }
                            };

                            match &branch.kind {
                                MatchBranchKind::Block { block, .. } => {
                                    for statement in block.inner.statements.iter() {
                                        let Statement::Expr { expr, .. } = statement else { continue };
                                        check_expr(expr);
                                    }

                                    if let Some(expr) = block.inner.final_expr_opt.as_ref() {
                                        check_expr(expr);
                                    }
                                }

                                MatchBranchKind::Expr { expr, .. } => {
                                    check_expr(expr);
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

                        while let Some(if_expr) = next_if_expr {
                            let IfExpr {
                                condition: IfCondition::Let { lhs, rhs, .. },
                                then_block,
                                else_opt,
                                ..
                            } = if_expr else {
                                break;
                            };

                            let mut check_if_expr = || {
                                // Check if `rhs` is a variable declaration, skip if so
                                if fn_state.expr_is_variable(rhs.as_ref(), context.blocks.as_slice()) {
                                    return;
                                }
                
                                // Check if `rhs` is a parameter of type `Identity`
                                let Some((address_checked, contract_id_checked)) = fn_state.find_identity_check(rhs.as_ref()) else { return };
                                
                                // Check `then_block` statements for `Identity::Address` and `Identity::ContractId` zero value checks
                                let Some((identity_kind, identity_value)) = utils::pattern_to_constructor_suffix_and_value("Identity", lhs.as_ref()) else { return };
    
                                let check_expr = |expr: &Expr| {
                                    let Expr::NotEqual { lhs, rhs, .. } = expr else { return };
    
                                    if utils::is_zero_value_comparison(identity_kind.as_str(), identity_value.span().as_str(), lhs.as_ref(), rhs.as_ref()) {
                                        match identity_kind.as_str() {
                                            "Address" => *address_checked.borrow_mut() = true,
                                            "ContractId" => *contract_id_checked.borrow_mut() = true,
                                            _ => {}
                                        }
                                    }
                                };
    
                                for statement in then_block.inner.statements.iter() {
                                    let Statement::Expr { expr, .. } = statement else { continue };
                                    check_expr(expr);
                                }
    
                                if let Some(expr) = then_block.inner.final_expr_opt.as_ref() {
                                    check_expr(expr);
                                }
                            };

                            check_if_expr();

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

#[cfg(test)]
mod tests {
    #[test]
    fn test_input_identity_validation() {
        crate::tests::test_detector("input_identity_validation")
    }
}
