use sway_ast::{attribute::{Attribute, Annotated}, *};
use sway_types::{BaseIdent, Span, Spanned};

pub fn fold_punctuated<T, P>(punctuated: &Punctuated<T, P>) -> Vec<&T> {
    let mut result = vec![];

    for (x, _) in punctuated.value_separator_pairs.iter() {
        result.push(x);
    }

    if let Some(x) = punctuated.final_value_opt.as_ref() {
        result.push(x.as_ref());
    }

    result
}

pub fn fold_tuple(tuple: &ExprTupleDescriptor) -> Vec<&Expr> {
    let mut result = vec![];
    
    match tuple {
        ExprTupleDescriptor::Nil => {},
        ExprTupleDescriptor::Cons { head, tail, .. } => {
            result.push(head.as_ref());
            result.extend(fold_punctuated(tail));
        },
    }

    result
}

pub fn fold_expr_ident_spans(expr: &Expr) -> Vec<Span> {
    let mut spans = vec![];

    match expr {
        Expr::Path(_) => {
            spans.push(expr.span());
        }

        Expr::AbiCast { args, .. } => {
            spans.extend(fold_expr_ident_spans(args.inner.address.as_ref()));
        }

        Expr::Struct { fields, .. } => {
            for field in fold_punctuated(&fields.inner) {
                if let Some(expr) = field.expr_opt.as_ref() {
                    spans.extend(fold_expr_ident_spans(expr.1.as_ref()));
                } else {
                    spans.push(field.field_name.span());
                }
            }
        }

        Expr::Tuple(tuple) => {
            if let ExprTupleDescriptor::Cons { head, tail, .. } = &tuple.inner {
                spans.extend(fold_expr_ident_spans(head.as_ref()));

                for expr in fold_punctuated(tail) {
                    spans.extend(fold_expr_ident_spans(expr));
                }
            }
        }

        Expr::Parens(expr) => {
            spans.extend(fold_expr_ident_spans(expr.inner.as_ref()));
        }
        
        Expr::Array(array) => {
            match &array.inner {
                ExprArrayDescriptor::Sequence(sequence) => {
                    for expr in fold_punctuated(sequence) {
                        spans.extend(fold_expr_ident_spans(expr));
                    }
                }

                ExprArrayDescriptor::Repeat { value, length, .. } => {
                    spans.extend(fold_expr_ident_spans(value.as_ref()));
                    spans.extend(fold_expr_ident_spans(length.as_ref()));
                }
            }
        }

        Expr::Return { expr_opt: Some(expr), .. } => {
            spans.extend(fold_expr_ident_spans(expr.as_ref()));
        }

        Expr::FuncApp { func, args } => {
            spans.extend(fold_expr_ident_spans(func.as_ref()));

            for arg in fold_punctuated(&args.inner) {
                spans.extend(fold_expr_ident_spans(arg));
            }
        }

        Expr::Index { target, arg } => {
            spans.extend(fold_expr_ident_spans(target.as_ref()));
            spans.extend(fold_expr_ident_spans(arg.inner.as_ref()));
        }

        Expr::MethodCall { target, args, .. } => {
            spans.extend(fold_expr_ident_spans(target.as_ref()));
            
            for arg in fold_punctuated(&args.inner) {
                spans.extend(fold_expr_ident_spans(arg));
            }
        }

        Expr::FieldProjection { target, .. } => {
            spans.push(expr.span());
            spans.extend(fold_expr_ident_spans(target.as_ref()));
        }

        Expr::TupleFieldProjection { target, .. } => {
            spans.push(expr.span());
            spans.extend(fold_expr_ident_spans(target.as_ref()));
        }

        Expr::Ref { expr, .. } => {
            spans.extend(fold_expr_ident_spans(expr.as_ref()));
        }

        Expr::Deref { expr, .. } => {
            spans.extend(fold_expr_ident_spans(expr.as_ref()));
        }

        Expr::Not { expr, .. } => {
            spans.extend(fold_expr_ident_spans(expr.as_ref()));
        }

        Expr::Mul { lhs, rhs, .. } => {
            spans.extend(fold_expr_ident_spans(lhs.as_ref()));
            spans.extend(fold_expr_ident_spans(rhs.as_ref()));
        }

        Expr::Div { lhs, rhs, .. } => {
            spans.extend(fold_expr_ident_spans(lhs.as_ref()));
            spans.extend(fold_expr_ident_spans(rhs.as_ref()));
        }

        Expr::Pow { lhs, rhs, .. } => {
            spans.extend(fold_expr_ident_spans(lhs.as_ref()));
            spans.extend(fold_expr_ident_spans(rhs.as_ref()));
        }

        Expr::Modulo { lhs, rhs, .. } => {
            spans.extend(fold_expr_ident_spans(lhs.as_ref()));
            spans.extend(fold_expr_ident_spans(rhs.as_ref()));
        }

        Expr::Add { lhs, rhs, .. } => {
            spans.extend(fold_expr_ident_spans(lhs.as_ref()));
            spans.extend(fold_expr_ident_spans(rhs.as_ref()));
        }

        Expr::Sub { lhs, rhs, .. } => {
            spans.extend(fold_expr_ident_spans(lhs.as_ref()));
            spans.extend(fold_expr_ident_spans(rhs.as_ref()));
        }

        Expr::Shl { lhs, rhs, .. } => {
            spans.extend(fold_expr_ident_spans(lhs.as_ref()));
            spans.extend(fold_expr_ident_spans(rhs.as_ref()));
        }

        Expr::Shr { lhs, rhs, .. } => {
            spans.extend(fold_expr_ident_spans(lhs.as_ref()));
            spans.extend(fold_expr_ident_spans(rhs.as_ref()));
        }

        Expr::BitAnd { lhs, rhs, .. } => {
            spans.extend(fold_expr_ident_spans(lhs.as_ref()));
            spans.extend(fold_expr_ident_spans(rhs.as_ref()));
        }

        Expr::BitXor { lhs, rhs, .. } => {
            spans.extend(fold_expr_ident_spans(lhs.as_ref()));
            spans.extend(fold_expr_ident_spans(rhs.as_ref()));
        }

        Expr::BitOr { lhs, rhs, .. } => {
            spans.extend(fold_expr_ident_spans(lhs.as_ref()));
            spans.extend(fold_expr_ident_spans(rhs.as_ref()));
        }

        Expr::Equal { lhs, rhs, .. } => {
            spans.extend(fold_expr_ident_spans(lhs.as_ref()));
            spans.extend(fold_expr_ident_spans(rhs.as_ref()));
        }

        Expr::NotEqual { lhs, rhs, .. } => {
            spans.extend(fold_expr_ident_spans(lhs.as_ref()));
            spans.extend(fold_expr_ident_spans(rhs.as_ref()));
        }

        Expr::LessThan { lhs, rhs, .. } => {
            spans.extend(fold_expr_ident_spans(lhs.as_ref()));
            spans.extend(fold_expr_ident_spans(rhs.as_ref()));
        }

        Expr::GreaterThan { lhs, rhs, .. } => {
            spans.extend(fold_expr_ident_spans(lhs.as_ref()));
            spans.extend(fold_expr_ident_spans(rhs.as_ref()));
        }

        Expr::LessThanEq { lhs, rhs, .. } => {
            spans.extend(fold_expr_ident_spans(lhs.as_ref()));
            spans.extend(fold_expr_ident_spans(rhs.as_ref()));
        }

        Expr::GreaterThanEq { lhs, rhs, .. } => {
            spans.extend(fold_expr_ident_spans(lhs.as_ref()));
            spans.extend(fold_expr_ident_spans(rhs.as_ref()));
        }

        Expr::LogicalAnd { lhs, rhs, .. } => {
            spans.extend(fold_expr_ident_spans(lhs.as_ref()));
            spans.extend(fold_expr_ident_spans(rhs.as_ref()));
        }

        Expr::LogicalOr { lhs, rhs, .. } => {
            spans.extend(fold_expr_ident_spans(lhs.as_ref()));
            spans.extend(fold_expr_ident_spans(rhs.as_ref()));
        }
        
        Expr::Reassignment { assignable, expr, .. } => {
            spans.push(assignable.span());
            spans.extend(fold_expr_ident_spans(expr.as_ref()));
        }
        
        _ => {}
    }

    spans
}

pub fn fold_path_idents(path: &PathExpr) -> Vec<BaseIdent> {
    let mut result = vec![];

    result.push(path.prefix.name.clone());

    for (_, path) in path.suffix.iter() {
        result.push(path.name.clone());
    }

    result
}

pub fn fold_expr_idents(expr: &Expr) -> Vec<BaseIdent> {
    let mut result = vec![];

    match expr {
        Expr::Path(path) => {
            result.extend(fold_path_idents(path));
        }

        Expr::Index { target, .. } => {
            result.extend(fold_expr_idents(target));
        }

        Expr::MethodCall { target, path_seg, .. } => {
            result.extend(fold_expr_idents(target));
            result.push(path_seg.name.clone());
        }

        Expr::FieldProjection { target, name, .. } => {
            result.extend(fold_expr_idents(target));
            result.push(name.clone());
        }

        Expr::TupleFieldProjection { target, .. } => {
            result.extend(fold_expr_idents(target));
        }

        _ => {}
    }

    result
}

pub fn fold_assignable_idents(assignable: &Assignable) -> Vec<BaseIdent> {
    let mut result = vec![];

    match assignable {
        Assignable::Var(ident) => {
            result.push(ident.clone());
        }

        Assignable::Index { target, .. } => {
            result.extend(fold_assignable_idents(target));
        }

        Assignable::FieldProjection { target, name, .. } => {
            result.extend(fold_assignable_idents(target));
            result.push(name.clone());
        }

        Assignable::TupleFieldProjection { target, .. } => {
            result.extend(fold_assignable_idents(target));
        }
    }

    result
}

pub fn fold_pattern_idents(pattern: &Pattern) -> Vec<BaseIdent> {
    let mut result = vec![];

    match pattern {
        Pattern::Or { lhs, rhs, .. } => {
            result.extend(fold_pattern_idents(lhs.as_ref()));
            result.extend(fold_pattern_idents(rhs.as_ref()));
        }

        Pattern::Wildcard { .. } => {}

        Pattern::AmbiguousSingleIdent(ident) => {
            result.push(ident.clone());
        }

        Pattern::Var { name, .. } => {
            result.push(name.clone());
        }

        Pattern::Literal(_) => {}

        Pattern::Constant(path) => {
            result.extend(fold_path_idents(path));
        }

        Pattern::Constructor { args, .. } => {
            //
            // NOTE: constructor path is ignored since it is a type name
            //

            for pattern in fold_punctuated(&args.inner) {
                result.extend(fold_pattern_idents(pattern));
            }
        }

        Pattern::Struct { fields, .. } => {
            //
            // NOTE: struct name is ignored since it is a type name
            //

            let mut fold_field_idents = |field: &PatternStructField| {
                match field {
                    PatternStructField::Rest { .. } => {}

                    PatternStructField::Field { field_name, pattern_opt } => {
                        match pattern_opt.as_ref() {
                            Some((_, pattern)) => {
                                result.extend(fold_pattern_idents(pattern));
                            }

                            None => {
                                result.push(field_name.clone());
                            }
                        }
                    }
                }
            };

            for field in fold_punctuated(&fields.inner) {
                fold_field_idents(field);
            }
        }

        Pattern::Tuple(patterns) => {
            for pattern in fold_punctuated(&patterns.inner) {
                result.extend(fold_pattern_idents(pattern));
            }
        }

        Pattern::Error(_, _) => {}
    }

    result
}

pub fn check_attribute_decls(
    attribute_decls: &[AttributeDecl],
    attribute_name: &str,
    attribute_arg_names: &[&str],
) -> bool {
    for attribute_decl in attribute_decls {
        for attribute in fold_punctuated(&attribute_decl.attribute.inner) {
            if check_attribute(attribute, attribute_name, attribute_arg_names) {
                return true;
            }
        }
    }
    
    false
}

#[inline]
fn check_attribute(
    attribute: &Attribute,
    attribute_name: &str,
    attribute_arg_names: &[&str],
) -> bool {
    if attribute.name.as_str() != attribute_name {
        return false;
    }

    if attribute_arg_names.is_empty() {
        return true;
    }

    let mut results = vec![];

    if let Some(args) = attribute.args.as_ref() {
        for &attribute_arg_name in attribute_arg_names {
            let mut result = false;

            for attribute_arg in fold_punctuated(&args.inner) {
                if attribute_arg.name.as_str() == attribute_arg_name {
                    result = true;
                    break;
                }
            }

            results.push(result);
        }
    }

    results.iter().all(|x| *x == true)
}

pub fn statement_to_variable_binding_ident(statement: &Statement) -> Option<BaseIdent> {
    let Statement::Let(StatementLet {
        pattern,
        ..
    }) = statement else { return None };
    
    let Pattern::Var {
        name: variable_name,
        ..
    } = pattern else { return None };
    
    Some(variable_name.clone())
}

pub fn find_storage_access_in_expr(expr: &Expr) -> Option<&Expr> {
    fn find_storage_access_in_block(block: &Braces<CodeBlockContents>) -> Option<&Expr> {
        for statement in block.inner.statements.iter() {
            match statement {
                Statement::Let(stmt_let) => {
                    let result = find_storage_access_in_expr(&stmt_let.expr);
                    if result.is_some() {
                        return result;
                    }
                }
                
                Statement::Item(_) => {},
                
                Statement::Expr { expr, .. } => {
                    let result = find_storage_access_in_expr(expr);
                    if result.is_some() {
                        return result;
                    }
                }

                Statement::Error(_, _) => {}
            }
        }

        if let Some(expr) = block.inner.final_expr_opt.as_ref() {
            let result = find_storage_access_in_expr(expr.as_ref());
            if result.is_some() {
                return result;
            }
        }

        None
    }

    fn find_storage_access_in_if_expr(if_expr: &IfExpr) -> Option<&Expr> {
        match &if_expr.condition {
            sway_ast::IfCondition::Expr(expr) => {
                let result = find_storage_access_in_expr(expr.as_ref());
                if result.is_some() {
                    return result;
                }
            }

            sway_ast::IfCondition::Let { rhs, .. } => {
                let result = find_storage_access_in_expr(rhs.as_ref());
                if result.is_some() {
                    return result;
                }
            }
        }

        let result = find_storage_access_in_block(&if_expr.then_block);
        if result.is_some() {
            return result;
        }

        match if_expr.else_opt.as_ref() {
            Some(else_if_expr) => match &else_if_expr.1 {
                sway_ast::expr::LoopControlFlow::Continue(else_if_expr) => find_storage_access_in_if_expr(else_if_expr.as_ref()),
                sway_ast::expr::LoopControlFlow::Break(else_block) => find_storage_access_in_block(else_block),
            }

            None => None,
        }
    }

    match expr {
        Expr::Error(_, _) => None,
        Expr::Path(_) => None,
        Expr::Literal(_) => None,
        Expr::AbiCast { args, .. } => find_storage_access_in_expr(args.inner.address.as_ref()),
        Expr::Struct { fields, .. } => {
            for field in fold_punctuated(&fields.inner) {
                if let Some(expr) = field.expr_opt.as_ref() {
                    let result = find_storage_access_in_expr(expr.1.as_ref());
                    if result.is_some() {
                        return result;
                    }
                }
            }
            None
        }
        Expr::Tuple(tuple) => {
            for expr in fold_tuple(&tuple.inner) {
                let result = find_storage_access_in_expr(expr);
                if result.is_some() {
                    return result;
                }
            }
            None
        }
        Expr::Parens(parens) => find_storage_access_in_expr(parens.inner.as_ref()),
        Expr::Block(block) => find_storage_access_in_block(block),
        Expr::Array(array) => {
            match &array.inner {
                sway_ast::ExprArrayDescriptor::Sequence(sequence) => {
                    for expr in fold_punctuated(sequence) {
                        let result = find_storage_access_in_expr(expr);
                        if result.is_some() {
                            return result;
                        }
                    }
                }
                sway_ast::ExprArrayDescriptor::Repeat { value, length, .. } => {
                    let result = find_storage_access_in_expr(value.as_ref());
                    if result.is_some() {
                        return result;
                    }
                    let result = find_storage_access_in_expr(length.as_ref());
                    if result.is_some() {
                        return result;
                    }
                }
            }
            None
        }
        Expr::Asm(_) => None,
        Expr::Return { expr_opt, .. } => {
            if let Some(expr) = expr_opt.as_ref() {
                let result = find_storage_access_in_expr(expr);
                if result.is_some() {
                    return result;
                }
            }
            None
        }
        Expr::If(if_expr) => find_storage_access_in_if_expr(if_expr),
        Expr::Match { value, branches, .. } => {
            let result = find_storage_access_in_expr(value.as_ref());
            if result.is_some() {
                return result;
            }

            for branch in branches.inner.iter() {
                match &branch.kind {
                    sway_ast::MatchBranchKind::Block { block, .. } => {
                        let result = find_storage_access_in_block(block);
                        if result.is_some() {
                            return result;
                        }
                    }

                    sway_ast::MatchBranchKind::Expr { expr, .. } => {
                        let result = find_storage_access_in_expr(expr);
                        if result.is_some() {
                            return result;
                        }
                    }
                }
            }

            None
        }
        Expr::While { condition, block, .. } => {
            let result = find_storage_access_in_expr(condition.as_ref());
            if result.is_some() {
                return result;
            }

            find_storage_access_in_block(block)
        }
        Expr::FuncApp { func, args } => {
            let result = find_storage_access_in_expr(func.as_ref());
            if result.is_some() {
                return result;
            }

            for arg in fold_punctuated(&args.inner) {
                let result = find_storage_access_in_expr(arg);
                if result.is_some() {
                    return result;
                }
            }

            None
        }
        Expr::Index { target, arg } => {
            let result = find_storage_access_in_expr(target.as_ref());
            if result.is_some() {
                return result;
            }

            find_storage_access_in_expr(arg.inner.as_ref())
        }
        Expr::MethodCall { target, contract_args_opt, args, .. } => {
            let idents = fold_expr_idents(expr);
            if idents.len() >= 3 {
                if idents.first().unwrap().as_str() == "storage" {
                    return Some(expr);
                }
            }

            let result = find_storage_access_in_expr(target.as_ref());
            if result.is_some() {
                return result;
            }

            if let Some(contract_args) = contract_args_opt.as_ref() {
                for contract_arg in fold_punctuated(&contract_args.inner) {
                    if let Some(expr) = contract_arg.expr_opt.as_ref() {
                        let result = find_storage_access_in_expr(expr.1.as_ref());
                        if result.is_some() {
                            return result;
                        }
                    }
                }
            }
            
            for arg in fold_punctuated(&args.inner) {
                let result = find_storage_access_in_expr(arg);
                if result.is_some() {
                    return result;
                }
            }

            None
        }
        Expr::FieldProjection { target, .. } => find_storage_access_in_expr(target.as_ref()),
        Expr::TupleFieldProjection { target, .. } => find_storage_access_in_expr(target.as_ref()),
        Expr::Ref { expr, .. } => find_storage_access_in_expr(expr.as_ref()),
        Expr::Deref { expr, .. } => find_storage_access_in_expr(expr.as_ref()),
        Expr::Not { expr, .. } => find_storage_access_in_expr(expr.as_ref()),
        Expr::Mul { lhs, rhs, .. } => {
            let result = find_storage_access_in_expr(lhs.as_ref());
            if result.is_some() {
                return result;
            }

            find_storage_access_in_expr(rhs.as_ref())
        }
        Expr::Div { lhs, rhs, .. } => {
            let result = find_storage_access_in_expr(lhs.as_ref());
            if result.is_some() {
                return result;
            }

            find_storage_access_in_expr(rhs.as_ref())
        }
        Expr::Pow { lhs, rhs, .. } => {
            let result = find_storage_access_in_expr(lhs.as_ref());
            if result.is_some() {
                return result;
            }

            find_storage_access_in_expr(rhs.as_ref())
        }
        Expr::Modulo { lhs, rhs, .. } => {
            let result = find_storage_access_in_expr(lhs.as_ref());
            if result.is_some() {
                return result;
            }

            find_storage_access_in_expr(rhs.as_ref())
        }
        Expr::Add { lhs, rhs, .. } => {
            let result = find_storage_access_in_expr(lhs.as_ref());
            if result.is_some() {
                return result;
            }

            find_storage_access_in_expr(rhs.as_ref())
        }
        Expr::Sub { lhs, rhs, .. } => {
            let result = find_storage_access_in_expr(lhs.as_ref());
            if result.is_some() {
                return result;
            }

            find_storage_access_in_expr(rhs.as_ref())
        }
        Expr::Shl { lhs, rhs, .. } => {
            let result = find_storage_access_in_expr(lhs.as_ref());
            if result.is_some() {
                return result;
            }

            find_storage_access_in_expr(rhs.as_ref())
        }
        Expr::Shr { lhs, rhs, .. } => {
            let result = find_storage_access_in_expr(lhs.as_ref());
            if result.is_some() {
                return result;
            }

            find_storage_access_in_expr(rhs.as_ref())
        }
        Expr::BitAnd { lhs, rhs, .. } => {
            let result = find_storage_access_in_expr(lhs.as_ref());
            if result.is_some() {
                return result;
            }

            find_storage_access_in_expr(rhs.as_ref())
        }
        Expr::BitXor { lhs, rhs, .. } => {
            let result = find_storage_access_in_expr(lhs.as_ref());
            if result.is_some() {
                return result;
            }

            find_storage_access_in_expr(rhs.as_ref())
        }
        Expr::BitOr { lhs, rhs, .. } => {
            let result = find_storage_access_in_expr(lhs.as_ref());
            if result.is_some() {
                return result;
            }

            find_storage_access_in_expr(rhs.as_ref())
        }
        Expr::Equal { lhs, rhs, .. } => {
            let result = find_storage_access_in_expr(lhs.as_ref());
            if result.is_some() {
                return result;
            }

            find_storage_access_in_expr(rhs.as_ref())
        }
        Expr::NotEqual { lhs, rhs, .. } => {
            let result = find_storage_access_in_expr(lhs.as_ref());
            if result.is_some() {
                return result;
            }

            find_storage_access_in_expr(rhs.as_ref())
        }
        Expr::LessThan { lhs, rhs, .. } => {
            let result = find_storage_access_in_expr(lhs.as_ref());
            if result.is_some() {
                return result;
            }

            find_storage_access_in_expr(rhs.as_ref())
        }
        Expr::GreaterThan { lhs, rhs, .. } => {
            let result = find_storage_access_in_expr(lhs.as_ref());
            if result.is_some() {
                return result;
            }

            find_storage_access_in_expr(rhs.as_ref())
        }
        Expr::LessThanEq { lhs, rhs, .. } => {
            let result = find_storage_access_in_expr(lhs.as_ref());
            if result.is_some() {
                return result;
            }

            find_storage_access_in_expr(rhs.as_ref())
        }
        Expr::GreaterThanEq { lhs, rhs, .. } => {
            let result = find_storage_access_in_expr(lhs.as_ref());
            if result.is_some() {
                return result;
            }

            find_storage_access_in_expr(rhs.as_ref())
        }
        Expr::LogicalAnd { lhs, rhs, .. } => {
            let result = find_storage_access_in_expr(lhs.as_ref());
            if result.is_some() {
                return result;
            }

            find_storage_access_in_expr(rhs.as_ref())
        }
        Expr::LogicalOr { lhs, rhs, .. } => {
            let result = find_storage_access_in_expr(lhs.as_ref());
            if result.is_some() {
                return result;
            }

            find_storage_access_in_expr(rhs.as_ref())
        }
        Expr::Reassignment { expr, .. } => find_storage_access_in_expr(expr.as_ref()),
        Expr::Break { .. } => None,
        Expr::Continue { .. } => None,
    }
}

pub fn statement_to_storage_read_binding_idents(statement: &Statement) -> Option<(BaseIdent, BaseIdent)> {
    let Statement::Let(StatementLet {
        pattern,
        expr,
        ..
    }) = statement else { return None };
    
    let Pattern::Var {
        mutable: Some(_),
        name: variable_name,
        ..
    } = pattern else { return None };
    
    let storage_idents = fold_expr_idents(expr);

    if storage_idents.len() < 3 {
        return None;
    }

    if storage_idents[0].as_str() != "storage" {
        return None;
    }

    if storage_idents.last().unwrap().as_str() != "read" {
        return None;
    }

    let storage_name = &storage_idents[1];

    Some((storage_name.clone(), variable_name.clone()))
}

pub fn statement_to_reassignment_idents(statement: &Statement) -> Option<Vec<BaseIdent>> {
    let Statement::Expr {
        expr,
        ..
    } = statement else { return None };

    let Expr::Reassignment {
        assignable,
        ..
    } = expr else { return None };
    
    Some(fold_assignable_idents(assignable))
}

fn is_storage_bytes_write_fn(s: &str) -> bool {
    matches!(s, "write_slice" | "clear")
}

fn is_storage_key_write_fn(s: &str) -> bool {
    matches!(s, "write")
}

fn is_storage_map_write_fn(s: &str) -> bool {
    matches!(s, "insert" | "remove")
}

fn is_storage_string_write_fn(s: &str) -> bool {
    matches!(s, "write_slice" | "clear")
}

fn is_storage_vec_write_fn(s: &str) -> bool {
    matches!(s, "push" | "pop" | "remove" | "swap_remove" | "set" | "insert" | "clear" | "swap" | "reverse" | "fill" | "resize")
}

pub fn storage_write_statement_to_storage_variable_ident(statement: &Statement) -> Option<BaseIdent> {
    let Statement::Expr { expr, .. } = statement else { return None };
    let Expr::MethodCall { .. } = expr else { return None };

    let storage_idents = fold_expr_idents(expr);

    if storage_idents.len() < 3 {
        return None;
    }

    if storage_idents[0].as_str() != "storage" {
        return None;
    }

    match storage_idents.last().unwrap().as_str() {
        s if is_storage_bytes_write_fn(s) => {}
        s if is_storage_key_write_fn(s) => {}
        s if is_storage_map_write_fn(s) => {}
        s if is_storage_string_write_fn(s) => {}
        s if is_storage_vec_write_fn(s) => {}
        _ => return None,
    }

    Some(storage_idents[1].clone())
}

pub fn statement_to_storage_write_idents(statement: &Statement) -> Option<(BaseIdent, BaseIdent)> {
    let Statement::Expr {
        expr,
        ..
    } = statement else { return None };

    let Expr::MethodCall {
        args,
        ..
    } = expr else { return None };

    let storage_idents = fold_expr_idents(expr);

    if storage_idents.len() < 3 {
        return None;
    }

    if storage_idents[0].as_str() != "storage" {
        return None;
    }

    match storage_idents.last().unwrap().as_str() {
        s if is_storage_bytes_write_fn(s) => {}
        s if is_storage_key_write_fn(s) => {}
        s if is_storage_map_write_fn(s) => {}
        s if is_storage_string_write_fn(s) => {}
        s if is_storage_vec_write_fn(s) => {}
        _ => return None,
    }

    let args = fold_punctuated(&args.inner);
    let Some(arg) = args.last() else { return None };
    let variable_idents = fold_expr_idents(arg);

    // TODO: need to support paths with multiple idents
    if variable_idents.len() != 1 {
        return None;
    }

    Some((storage_idents[1].clone(), variable_idents[0].clone()))
}

pub fn block_has_revert(block: &Braces<CodeBlockContents>) -> bool {
    // Check if `if_expr.then_block` contains a revert
    let mut has_revert = false;

    for statement in block.inner.statements.iter() {
        let Statement::Expr { expr, .. } = statement else { continue };
        let Expr::FuncApp { func, .. } = expr else { continue };
        
        if let "revert" = func.span().as_str() {
            has_revert = true;
            break;
        }
    }

    if let Some(expr) = block.inner.final_expr_opt.as_ref() {
        if let Expr::FuncApp { func, .. } = expr.as_ref() {
            if let "revert" = func.span().as_str() {
                has_revert = true;
            }
        }
    }

    has_revert
}

pub fn is_zero_value_comparison(type_name: &str, var_name: &str, lhs: &Expr, rhs: &Expr) -> bool {
    let zero_value = if lhs.span().as_str() == var_name {
        rhs
    } else if rhs.span().as_str() == var_name {
        lhs
    } else {
        return false;
    };

    let Expr::FuncApp { func, args } = zero_value else { return false };
    if func.span().as_str() != format!("{type_name}::from") { return false; }
    if args.span().as_str() != "(ZERO_B256)" { return false; }

    true
}

pub fn get_require_args(expr: &Expr) -> Option<Vec<&Expr>> {
    let Expr::FuncApp { func, args } = expr else { return None };
    let "require" = func.span().as_str() else { return None };
    Some(fold_punctuated(&args.inner))
}

pub fn pattern_to_constructor_suffix_and_value(name: &str, pattern: &Pattern) -> Option<(BaseIdent, BaseIdent)> {
    let Pattern::Constructor { path, args } = pattern else { return None };
    if path.prefix.name.as_str() != name { return None; }
    let Some(suffix) = path.suffix.last() else { return None };
    let args = fold_punctuated(&args.inner);
    let Some(Pattern::AmbiguousSingleIdent(ident)) = args.first() else { return None };
    Some((suffix.1.name.clone(), ident.clone()))
}

pub fn collect_storage_fields(module: &Module) -> Vec<&StorageField> {
    let Some(Annotated {
        value: ItemKind::Storage(storage),
        ..
    }) = module.items.iter().find(|x| matches!(x.value, ItemKind::Storage(_))) else { return vec![] };

    fold_punctuated(&storage.fields.inner).iter().map(|x| &x.value).collect()
}

pub fn is_boolean_literal_or_negation(expr: &Expr) -> bool {
    match expr {
        Expr::Literal(x) => {
            let x = x.span();
            x.as_str() == "true" || x.as_str() == "false"
        }

        Expr::Not { expr, .. } => is_boolean_literal_or_negation(expr),

        _ => false,
    }
}

pub fn get_item_location(item: &ItemKind, item_impl: &Option<&ItemImpl>, item_fn: &Option<&ItemFn>) -> String {
    match item {
        ItemKind::Fn(item_fn) => if let Some(item_impl) = item_impl.as_ref() {
            format!(
                "The `{}::{}` function",
                item_impl.ty.span().as_str(),
                item_fn.fn_signature.name.as_str(),
            )
        } else {
            format!(
                "The `{}` function",
                item_fn.fn_signature.name.as_str(),
            )
        },

        ItemKind::Const(item_const) => match (item_impl.as_ref(), item_fn.as_ref()) {
            (Some(item_impl), Some(item_fn)) => format!(
                "The `{}` constant in the `{}::{}` function",
                item_const.name,
                item_impl.ty.span().as_str(),
                item_fn.fn_signature.name,
            ),

            (Some(item_impl), None) => format!(
                "The `{}::{}` constant",
                item_impl.ty.span().as_str(),
                item_const.name.as_str(),
            ),

            (None, Some(item_fn)) => format!(
                "The `{}` constant in the `{}` function",
                item_const.name,
                item_fn.fn_signature.name,
            ),

            (None, None) => format!(
                "The `{}` constant",
                item_const.name,
            ),
        },

        ItemKind::Storage(_) => format!("Storage"),
        ItemKind::Configurable(_) => format!("Configurable"),
        
        _ => panic!("Unhandled item location: {:#?}", item),
    }
}

pub fn use_tree_to_name(mut use_tree: &UseTree, path: &str) -> Option<String> {
    let tokens = path.split("::").collect::<Vec<_>>();
    
    if tokens.len() > 1 {
        let prefixes = &tokens[..tokens.len() - 1];

        for p in prefixes {
            match use_tree {
                UseTree::Group { imports } => {
                    let mut has_prefix = false;

                    for import in &imports.inner {
                        match import {
                            UseTree::Path { prefix, suffix, .. } if prefix.as_str() == *p => {
                                use_tree = suffix.as_ref();
                                has_prefix = true;
                                break;
                            }

                            _ => {}
                        }
                    }

                    if !has_prefix {
                        return None;
                    }
                }
                
                UseTree::Path { prefix, suffix, .. } if prefix.as_str() == *p => {
                    use_tree = suffix.as_ref();
                }

                _ => return None,
            }
        }
    }

    let Some(import_name) = tokens.last() else { return None };

    match use_tree {
        UseTree::Name { name } if name.as_str() == *import_name => {
            Some(name.as_str().to_string())
        }

        UseTree::Rename { name, alias, .. } if name.as_str() == *import_name => {
            Some(alias.as_str().to_string())
        }

        UseTree::Group { imports } => {
            for import in &imports.inner {
                match import {
                    UseTree::Name { name } if name.as_str() == *import_name => {
                        return Some(name.as_str().to_string());
                    }
            
                    UseTree::Rename { name, alias, .. } if name.as_str() == *import_name => {
                        return Some(alias.as_str().to_string());
                    }

                    _ => {}
                }
            }

            None
        }

        _ => None,
    }
}

pub fn expr_binary_operands(expr: &Expr) -> Option<(&Expr, &Expr)> {
    match expr {
        Expr::Mul { lhs, rhs, .. } => Some((lhs.as_ref(), rhs.as_ref())),
        Expr::Div { lhs, rhs, .. } => Some((lhs.as_ref(), rhs.as_ref())),
        Expr::Pow { lhs, rhs, .. } => Some((lhs.as_ref(), rhs.as_ref())),
        Expr::Modulo { lhs, rhs, .. } => Some((lhs.as_ref(), rhs.as_ref())),
        Expr::Add { lhs, rhs, .. } => Some((lhs.as_ref(), rhs.as_ref())),
        Expr::Sub { lhs, rhs, .. } => Some((lhs.as_ref(), rhs.as_ref())),
        Expr::Shl { lhs, rhs, .. } => Some((lhs.as_ref(), rhs.as_ref())),
        Expr::Shr { lhs, rhs, .. } => Some((lhs.as_ref(), rhs.as_ref())),
        Expr::BitAnd { lhs, rhs, .. } => Some((lhs.as_ref(), rhs.as_ref())),
        Expr::BitXor { lhs, rhs, .. } => Some((lhs.as_ref(), rhs.as_ref())),
        Expr::BitOr { lhs, rhs, .. } => Some((lhs.as_ref(), rhs.as_ref())),
        Expr::Equal { lhs, rhs, .. } => Some((lhs.as_ref(), rhs.as_ref())),
        Expr::NotEqual { lhs, rhs, .. } => Some((lhs.as_ref(), rhs.as_ref())),
        Expr::LessThan { lhs, rhs, .. } => Some((lhs.as_ref(), rhs.as_ref())),
        Expr::GreaterThan { lhs, rhs, .. } => Some((lhs.as_ref(), rhs.as_ref())),
        Expr::LessThanEq { lhs, rhs, .. } => Some((lhs.as_ref(), rhs.as_ref())),
        Expr::GreaterThanEq { lhs, rhs, .. } => Some((lhs.as_ref(), rhs.as_ref())),
        Expr::LogicalAnd { lhs, rhs, .. } => Some((lhs.as_ref(), rhs.as_ref())),
        Expr::LogicalOr { lhs, rhs, .. } => Some((lhs.as_ref(), rhs.as_ref())),
        _ => None,
    }
}
