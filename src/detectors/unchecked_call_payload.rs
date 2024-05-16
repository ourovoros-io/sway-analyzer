use crate::{
    error::Error,
    project::Project,
    report::Severity,
    scope::AstScope,
    utils,
    visitor::{
        AsmBlockContext, AsmInstructionContext, AstVisitor, BlockContext, ExprContext, FnContext,
        IfExprContext, ModuleContext, StatementLetContext, UseContext,
    },
};
use std::{cell::RefCell, collections::HashMap, path::PathBuf, rc::Rc};
use sway_ast::{Expr, FnArgs, IfCondition, Pattern};
use sway_types::{Span, Spanned};

#[derive(Default)]
pub struct UncheckedCallPayloadVisitor {
    module_states: HashMap<PathBuf, ModuleState>,
}

struct ModuleState {
    fn_states: HashMap<Span, FnState>,
    bytes_type_names: Vec<String>,
}

impl Default for ModuleState {
    fn default() -> Self {
        Self {
            fn_states: Default::default(),
            bytes_type_names: vec![
                "std::bytes::Bytes".into(),
            ],
        }
    }
}

#[derive(Default)]
struct FnState {
    raw_ptr_arg_states: Vec<RawPtrArgState>,
    bytes_arg_states: Vec<BytesArgState>,
    block_states: HashMap<Span, BlockState>,
}

struct RawPtrArgState {
    ident_span: Span,
}

struct BytesArgState {
    ident_span: Span,
    type_name: String,
    len_checked: bool,
}

#[derive(Default)]
struct BlockState {
    vars: Vec<String>,
    asm_block_states: HashMap<Span, AsmBlockState>,
}

#[derive(Default)]
struct AsmBlockState {
    raw_ptr_arg_registers: Vec<(String, Span)>,
    bytes_arg_registers: Vec<(String, Span)>,
}

impl AstVisitor for UncheckedCallPayloadVisitor {
    fn visit_module(&mut self, context: &ModuleContext, _scope: Rc<RefCell<AstScope>>, _project: &mut Project) -> Result<(), Error> {
        // Create the module state
        if !self.module_states.contains_key(context.path) {
            self.module_states.insert(context.path.into(), ModuleState::default());
        }

        Ok(())
    }

    fn visit_use(&mut self, context: &UseContext, _scope: Rc<RefCell<AstScope>>, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Check use tree for `std::bytes::Bytes`
        if let Some(name) = utils::use_tree_to_name(&context.item_use.tree, "std::bytes::Bytes") {
            module_state.bytes_type_names.push(name);
        }
        
        Ok(())
    }

    fn visit_fn(&mut self, context: &FnContext, _scope: Rc<RefCell<AstScope>>, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Create the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.entry(fn_signature).or_default();

        let args = match &context.item_fn.fn_signature.arguments.inner {
            FnArgs::Static(args) => args,
            FnArgs::NonStatic { args_opt: Some(args), .. } => &args.1,
            _ => return Ok(()),
        };
        
        // Check for arguments that are of type `raw_ptr` or `Bytes`
        for arg in args {
            let Pattern::AmbiguousSingleIdent(ident) = &arg.pattern else { continue };

            match arg.ty.span().as_str() {
                "raw_ptr" => {
                    fn_state.raw_ptr_arg_states.push(RawPtrArgState {
                        ident_span: ident.span(),
                    });
                }
                
                type_name if module_state.bytes_type_names.contains(&type_name.into()) => {
                    fn_state.bytes_arg_states.push(BytesArgState {
                        ident_span: ident.span(),
                        type_name: type_name.into(),
                        len_checked: false,
                    });
                }

                _ => {}
            }
        }
        
        Ok(())
    }

    fn visit_block(&mut self, context: &BlockContext, _scope: Rc<RefCell<AstScope>>, _project: &mut Project) -> Result<(), Error> {
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

    fn visit_statement_let(&mut self, context: &StatementLetContext, _scope: Rc<RefCell<AstScope>>, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get_mut(&fn_signature).unwrap();

        // Get the block state
        let block_span = context.blocks.last().unwrap();
        let block_state = fn_state.block_states.get_mut(block_span).unwrap();

        // Store variables declared in the `let` pattern in the block state
        for ident in utils::fold_pattern_idents(&context.statement_let.pattern) {
            block_state.vars.push(ident.as_str().into());
        }

        Ok(())
    }

    fn visit_if_expr(&mut self, context: &IfExprContext, _scope: Rc<RefCell<AstScope>>, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get_mut(&fn_signature).unwrap();

        // Get or create the `if` expression's body block state
        let block_span = context.if_expr.then_block.span();
        let block_state = fn_state.block_states.entry(block_span).or_default();

        // Only check `if let` expressions
        let IfCondition::Let { lhs, .. } = &context.if_expr.condition else { return Ok(()) };

        // Store variables declared in the `let` pattern in the `if` expression's body block state
        for ident in utils::fold_pattern_idents(lhs.as_ref()) {
            block_state.vars.push(ident.as_str().into());
        }

        Ok(())
    }

    fn visit_expr(&mut self, context: &ExprContext, _scope: Rc<RefCell<AstScope>>, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Get the function state
        let Some(item_fn) = context.item_fn.as_ref() else { return Ok(()) };
        let fn_signature = item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get_mut(&fn_signature).unwrap();

        // Check for `require` or `if`-`revert` conditions
        let (expr, value) = if let Some(args) = utils::get_require_args(context.expr) {
            let Some(expr) = args.first() else { return Ok(()) };
            
            match expr {
                Expr::GreaterThanEq { lhs, rhs, .. } => (lhs.as_ref(), rhs.as_ref()),
                Expr::LessThanEq { lhs, rhs, .. } => (rhs.as_ref(), lhs.as_ref()),
                _ => return Ok(()),
            }
        } else if let Some(IfCondition::Expr(expr)) = utils::get_if_revert_condition(context.expr) {
            match expr.as_ref() {
                Expr::LessThan { lhs, rhs, .. } => (lhs.as_ref(), rhs.as_ref()),
                Expr::GreaterThan { lhs, rhs, .. } => (rhs.as_ref(), lhs.as_ref()),
                _ => return Ok(()),
            }
        } else {
            return Ok(());
        };

        let "32" = value.span().as_str() else { return Ok(()) };

        if let Some(arg_state) = fn_state.bytes_arg_states.iter_mut().find(|arg_state| expr.span().as_str() == format!("{}.len()", arg_state.ident_span.as_str())) {
            arg_state.len_checked = true;
        }

        Ok(())
    }

    fn visit_asm_block(&mut self, context: &AsmBlockContext, _scope: Rc<RefCell<AstScope>>, _project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get_mut(&fn_signature).unwrap();

        // Track `raw_ptr` and `Bytes` arguments that are used in registers
        for register in &context.asm.registers.inner {
            let is_arg_shadowed = |ident_span: &Span| -> bool {
                for block_span in context.blocks.iter().rev() {
                    let block_state = fn_state.block_states.get(block_span).unwrap();
                    
                    if block_state.vars.iter().any(|var| var == ident_span.as_str()) {
                        return true;
                    }
                }

                false
            };

            let mut arg_info: Option<(bool, Span)> = None; // (is_raw_ptr, arg_ident_span)

            if let Some((_, value)) = register.value_opt.as_ref() {
                // Check if `value` is a `raw_ptr` argument
                if let Some(arg_state) = fn_state.raw_ptr_arg_states.iter().find(|arg_state| value.span().as_str() == arg_state.ident_span.as_str()) {
                    if is_arg_shadowed(&arg_state.ident_span) {
                        continue;
                    }
                    
                    arg_info = Some((true, arg_state.ident_span.clone()));
                }
                // Check if `value` is a `Bytes` argument
                else if let Some(arg_state) = fn_state.bytes_arg_states.iter().find(|arg_state| value.span().as_str() == format!("{}.buf.ptr", arg_state.ident_span.as_str())) {
                    if is_arg_shadowed(&arg_state.ident_span) {
                        continue;
                    }
                    
                    arg_info = Some((false, arg_state.ident_span.clone()));
                }
            }

            if let Some((is_raw_ptr, arg_ident_span)) = arg_info {
                // Get the current block state
                let block_span = context.blocks.last().unwrap();
                let block_state = fn_state.block_states.get_mut(block_span).unwrap();

                // Get or create the `asm` block state
                let asm_block_span = context.asm.span();
                let asm_block_state = block_state.asm_block_states.entry(asm_block_span).or_default();

                // Track the argument's register association
                if is_raw_ptr {
                    asm_block_state.raw_ptr_arg_registers.push((register.register.as_str().into(), arg_ident_span));
                } else {
                    asm_block_state.bytes_arg_registers.push((register.register.as_str().into(), arg_ident_span));
                }
            }
        }

        Ok(())
    }

    fn visit_asm_instruction(&mut self, context: &AsmInstructionContext, _scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        // Get the module state
        let module_state = self.module_states.get_mut(context.path).unwrap();

        // Get the function state
        let fn_signature = context.item_fn.fn_signature.span();
        let fn_state = module_state.fn_states.get_mut(&fn_signature).unwrap();

        // Get the current block state
        let block_span = context.blocks.last().unwrap();
        let block_state = fn_state.block_states.get_mut(block_span).unwrap();

        // Get or create the `asm` block state
        let asm_block_span = context.asm.span();
        let asm_block_state = block_state.asm_block_states.entry(asm_block_span).or_default();

        // Only check `CALL` instructions
        let "call" = context.instruction.op_code_ident().as_str() else { return Ok(()) };
        let call_span = context.instruction.span();
        let call_register_arg_idents = context.instruction.register_arg_idents();

        if call_register_arg_idents.is_empty() {
            return Ok(());
        }

        // Check if first used register is associated with a `raw_ptr` argument
        if let Some((_, arg_ident_span)) = asm_block_state.raw_ptr_arg_registers.iter().find(|(register, _)| register == call_register_arg_idents[0].as_str()) {
            project.report.borrow_mut().add_entry(
                context.path,
                project.span_to_line(context.path, &call_span)?,
                Severity::Low,
                format!(
                    "{} uses the `{}: raw_ptr` parameter as the payload in a `CALL` instruction via register `{}`, which may revert if the data is incorrect: `{}`",
                    utils::get_item_location(context.item, &context.item_impl, &Some(context.item_fn)),
                    arg_ident_span.as_str(),
                    call_register_arg_idents[0].as_str(),
                    call_span.as_str(),
                ),
            );
        }
        // Check if first used register is associated with a `Bytes` argument
        else if let Some((_, arg_ident_span)) = asm_block_state.bytes_arg_registers.iter().find(|(register, _)| register == call_register_arg_idents[0].as_str()) {
            let Some(arg_state) = fn_state.bytes_arg_states.iter().find(|arg_state| *arg_ident_span == arg_state.ident_span) else { return Ok(()) };

            if !arg_state.len_checked {
                project.report.borrow_mut().add_entry(
                    context.path,
                    project.span_to_line(context.path, &call_span)?,
                    Severity::Low,
                    format!(
                        "{} uses the `{}: {}` parameter as the payload in a `CALL` instruction via register `{}` without checking its length, which may revert if the data is incorrect: `{}`",
                        utils::get_item_location(context.item, &context.item_impl, &Some(context.item_fn)),
                        arg_state.ident_span.as_str(),
                        arg_state.type_name,
                        call_register_arg_idents[0].as_str(),
                        call_span.as_str(),
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
    fn test_unchecked_call_payload() {
        crate::tests::test_detector("unchecked_call_payload", 2);
    }
}
