use sway_ast::{assignable::ElementAccess, attribute::{Annotated, Attribute}, ty::{TyArrayDescriptor, TyTupleDescriptor}, *};
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

pub fn fold_element_access_idents(element_access: &ElementAccess) -> Vec<BaseIdent> {
    let mut result = vec![];

    match element_access {
        ElementAccess::Var(ident) => {
            result.push(ident.clone());
        }

        ElementAccess::Index { target, .. } => {
            result.extend(fold_element_access_idents(target));
        }

        ElementAccess::FieldProjection { target, name, .. } => {
            result.extend(fold_element_access_idents(target));
            result.push(name.clone());
        }

        ElementAccess::TupleFieldProjection { target, .. } => {
            result.extend(fold_element_access_idents(target));
        }
    }

    result
}

pub fn fold_assignable_idents(assignable: &Assignable) -> Vec<BaseIdent> {
    let mut result = vec![];

    match assignable {
        Assignable::ElementAccess(element_access) => {
            result.extend(fold_element_access_idents(element_access));
        }

        Assignable::Deref { expr, .. } => {
            result.extend(fold_expr_idents(expr));
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

pub fn map_expr<F: FnMut(&Expr)>(expr: &Expr, f: &mut F) {
    f(expr);

    match expr {
        Expr::Struct { fields, .. } => {
            for field in &fields.inner {
                if let Some((_, expr)) = field.expr_opt.as_ref() {
                    map_expr(expr.as_ref(), f);
                }
            }
        }

        Expr::Tuple(tuple) => {
            match &tuple.inner {
                ExprTupleDescriptor::Nil => {}
                ExprTupleDescriptor::Cons { head, tail, .. } => {
                    map_expr(head.as_ref(), f);

                    for expr in tail {
                        map_expr(expr, f);
                    }
                }
            }
        }
        
        Expr::Array(array) => {
            match &array.inner {
                ExprArrayDescriptor::Sequence(sequence) => {
                    for expr in sequence {
                        map_expr(expr, f);
                    }
                }
                ExprArrayDescriptor::Repeat { value, length, .. } => {
                    map_expr(value.as_ref(), f);
                    map_expr(length.as_ref(), f);
                }
            }
        }

        Expr::If(if_expr) => {
            match &if_expr.condition {
                IfCondition::Expr(expr) => map_expr(expr.as_ref(), f),
                IfCondition::Let { rhs, .. } => map_expr(rhs.as_ref(), f),
            }
        }

        Expr::FuncApp { func, args } => {
            map_expr(func.as_ref(), f);

            for arg in &args.inner {
                map_expr(arg, f);
            }
        }

        Expr::Index { target, arg } => {
            map_expr(target.as_ref(), f);
            map_expr(arg.inner.as_ref(), f);
        }

        Expr::MethodCall { target, contract_args_opt, args, .. } => {
            map_expr(target.as_ref(), f);

            if let Some(contract_args) = contract_args_opt.as_ref() {
                for arg in &contract_args.inner {
                    if let Some((_, expr)) = arg.expr_opt.as_ref() {
                        map_expr(expr.as_ref(), f);
                    }
                }
            }

            for arg in &args.inner {
                map_expr(arg, f);
            }
        }
        
        Expr::AbiCast { args: Parens { inner: AbiCastArgs { address: expr, .. }, .. }, .. } |
        Expr::Parens(Parens { inner: expr, .. }) |
        Expr::Return { expr_opt: Some(expr), .. } |
        Expr::Match { value: expr, .. } |
        Expr::While { condition: expr, .. } |
        Expr::FieldProjection { target: expr, .. } |
        Expr::TupleFieldProjection { target: expr, .. } |
        Expr::Ref { expr, .. } |
        Expr::Deref { expr, .. } |
        Expr::Not { expr, .. } => {
            map_expr(expr.as_ref(), f);
        }

        Expr::Mul { lhs, rhs, .. } |
        Expr::Div { lhs, rhs, .. } |
        Expr::Pow { lhs, rhs, .. } |
        Expr::Modulo { lhs, rhs, .. } |
        Expr::Add { lhs, rhs, .. } |
        Expr::Sub { lhs, rhs, .. } |
        Expr::Shl { lhs, rhs, .. } |
        Expr::Shr { lhs, rhs, .. } |
        Expr::BitAnd { lhs, rhs, .. } |
        Expr::BitXor { lhs, rhs, .. } |
        Expr::BitOr { lhs, rhs, .. } |
        Expr::Equal { lhs, rhs, .. } |
        Expr::NotEqual { lhs, rhs, .. } |
        Expr::LessThan { lhs, rhs, .. } |
        Expr::GreaterThan { lhs, rhs, .. } |
        Expr::LessThanEq { lhs, rhs, .. } |
        Expr::GreaterThanEq { lhs, rhs, .. } |
        Expr::LogicalAnd { lhs, rhs, .. } |
        Expr::LogicalOr { lhs, rhs, .. } => {
            map_expr(lhs.as_ref(), f);
            map_expr(rhs.as_ref(), f);
        }

        Expr::Reassignment { assignable, expr, .. } => {
            if let Assignable::ElementAccess(ElementAccess::Index { arg, .. }) = assignable { map_expr(arg.inner.as_ref(), f) }
            map_expr(expr.as_ref(), f);
        }

        _ => {}
    }
}

pub fn map_pattern<F: FnMut(&Pattern)>(pattern: &Pattern, f: &mut F) {
    f(pattern);

    match pattern {
        Pattern::Or { lhs, rhs, .. } => {
            map_pattern(lhs.as_ref(), f);
            map_pattern(rhs.as_ref(), f);
        }

        Pattern::Wildcard { .. } => {}
        Pattern::AmbiguousSingleIdent(_) => {}
        Pattern::Var { .. } => {}
        Pattern::Literal(_) => {}
        Pattern::Constant(_) => {}
        
        Pattern::Constructor { args, .. } => {
            for arg in &args.inner {
                map_pattern(arg, f);
            }
        }

        Pattern::Struct { fields, .. } => {
            for field in &fields.inner {
                if let PatternStructField::Field { pattern_opt: Some((_, pattern)), .. } = field {
                    map_pattern(pattern.as_ref(), f);
                }
            }
        }

        Pattern::Tuple(tuple) => {
            for pattern in &tuple.inner {
                map_pattern(pattern, f);
            }
        }

        Pattern::Error(_, _) => {}
    }
}

pub fn map_pattern_and_ty<F: FnMut(&Pattern, &Ty)>(pattern: &Pattern, ty: &Ty, f: &mut F) {
    f(pattern, ty);

    match pattern {
        Pattern::Or { lhs, rhs, .. } => {
            map_pattern_and_ty(lhs.as_ref(), ty, f);
            map_pattern_and_ty(rhs.as_ref(), ty, f);
        }

        Pattern::Wildcard { .. } => {}
        Pattern::AmbiguousSingleIdent(_) => {}
        Pattern::Var { .. } => {}
        Pattern::Literal(_) => {}
        Pattern::Constant(_) => {}
        
        Pattern::Constructor { args, .. } => {
            // TODO: We need to find the type declaration to resolve the arg types
            // for arg in &args.inner {
            //     map_pattern_and_ty(arg, arg.ty, f);
            // }
        }

        Pattern::Struct { fields, .. } => {
            // TODO: We need to find the type declaration to resolve the field types
            // for field in &fields.inner {
            //     if let PatternStructField::Field { pattern_opt: Some((_, pattern)), .. } = field {
            //         map_pattern_and_ty(pattern.as_ref(), f);
            //     }
            // }
        }

        Pattern::Tuple(tuple) => {
            let mut types: Vec<&Ty> = vec![];

            match ty {
                Ty::Tuple(ty) => {
                    match &ty.inner {
                        ty::TyTupleDescriptor::Nil => {}

                        ty::TyTupleDescriptor::Cons { head, tail, .. } => {
                            types.push(head.as_ref());
                            
                            for ty in tail {
                                types.push(ty);
                            }
                        }
                    }
                }

                _ => todo!(),
            }

            let mut patterns: Vec<&Pattern> = vec![];
            
            for pattern in &tuple.inner {
                patterns.push(pattern);
            }

            for (pattern, ty) in patterns.iter().zip(types.iter()) {
                map_pattern_and_ty(pattern, ty, f);
            }
        }

        Pattern::Error(_, _) => {}
    }
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

    results.iter().all(|x| *x)
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
        Expr::For { iterator, block, .. } => {
            let result = find_storage_access_in_expr(iterator.as_ref());
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
            if idents.len() >= 3 && idents.first().unwrap().as_str() == "storage" {
                return Some(expr);
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
    let arg = args.last()?;
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

pub fn get_if_revert_condition(expr: &Expr) -> Option<&IfCondition> {
    let Expr::If(IfExpr { condition, then_block, .. }) = expr else { return None };
    if !block_has_revert(then_block) { return None; }
    Some(condition)
}

pub fn pattern_to_constructor_suffix_and_value(name: &str, pattern: &Pattern) -> Option<(BaseIdent, BaseIdent)> {
    let Pattern::Constructor { path, args } = pattern else { return None };
    if path.prefix.name.as_str() != name { return None; }
    let suffix = path.suffix.last()?;    
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

        ItemKind::Storage(_) => "Storage".to_string(),
        ItemKind::Configurable(_) => "Configurable".to_string(),
        
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

                _ => {}
            }
        }
    }

    let import_name = tokens.last()?;

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

pub fn flatten_use_tree(prefix: Option<&PathExpr>, use_tree: &UseTree) -> Vec<PathExpr> {
    match use_tree {
        UseTree::Group { imports } => {
            let mut result = vec![];
            for import in &imports.inner {
                result.extend(flatten_use_tree(prefix, import));
            }
            result
        },
        UseTree::Name { name } => {
            let mut prefix = prefix.cloned();
            if prefix.is_none() {
                prefix = Some(PathExpr { 
                    root_opt: None, 
                    prefix: PathExprSegment { 
                        name: name.clone(), 
                        generics_opt: None 
                    }, 
                    suffix: vec![], 
                    incomplete_suffix: false 
                });
            } else {
                prefix.as_mut().unwrap().suffix.push((DoubleColonToken::default(), PathExprSegment { 
                    name: name.clone(), 
                    generics_opt: None 
                }));
            }

            if prefix.as_ref().map(|x| x.suffix.last().map(|l| l.1.name.as_str() == "self").unwrap_or(false)).unwrap_or(false) {
                prefix.as_mut().unwrap().suffix.pop();
            } 
            vec![prefix.unwrap()]
        },
        UseTree::Rename { alias, .. } => {
            let prefix = PathExpr { 
                root_opt: None, 
                prefix: PathExprSegment { 
                    name: alias.clone(), 
                    generics_opt: None 
                }, 
                suffix: vec![], 
                incomplete_suffix: false 
            };
            vec![prefix]
        },
        UseTree::Glob { .. } => {
            let mut prefix = prefix.cloned();
            let name = BaseIdent::new_no_span("*".to_string());
            if prefix.is_none() {
                prefix = Some(PathExpr { 
                    root_opt: None, 
                    prefix: PathExprSegment { 
                        name: name.clone(), 
                        generics_opt: None 
                    }, 
                    suffix: vec![], 
                    incomplete_suffix: false 
                });
            } else {
                prefix.as_mut().unwrap().suffix.push((DoubleColonToken::default(), PathExprSegment { 
                    name: name.clone(), 
                    generics_opt: None 
                }));
            }

            if prefix.as_ref().map(|x| x.suffix.last().map(|l| l.1.name.as_str() == "self").unwrap_or(false)).unwrap_or(false) {
                prefix.as_mut().unwrap().suffix.pop();
            } 
            vec![prefix.unwrap()]
        },
        UseTree::Path { prefix: inner_prefix, suffix , .. } => {
            let mut prefix = prefix.cloned();
            if prefix.is_none() {
                prefix = Some(PathExpr { 
                    root_opt: None, 
                    prefix: PathExprSegment { 
                        name: inner_prefix.clone(), 
                        generics_opt: None 
                    }, 
                    suffix: vec![], 
                    incomplete_suffix: false 
                });
            } else {
                prefix.as_mut().unwrap().suffix.push((DoubleColonToken::default(), PathExprSegment { 
                    name: inner_prefix.clone(), 
                    generics_opt: None 
                }));
            }

            flatten_use_tree(prefix.as_ref(), suffix)
        },
        UseTree::Error { .. } => panic!("Encountered error while expanding use tree"),
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

pub fn expr_negation_result(expr: &Expr) -> bool {
    match expr {
        Expr::Parens(expr) => expr_negation_result(expr.inner.as_ref()),
        Expr::Not { expr, .. } => !expr_negation_result(expr.as_ref()),
        _ => true,
    }
}

pub fn ty_to_string(ty: &Ty) -> String {
    match ty {
        Ty::Path(path_type) => path_type_to_string(path_type),
        Ty::Tuple(tuple) => ty_tuple_descriptor_to_string(&tuple.inner),
        Ty::Array(array) => ty_array_descriptor_to_string(&array.inner),
        Ty::StringSlice(_) => "str".into(),
        Ty::StringArray { length, .. } => format!("str[{}]", length.inner.span().as_str()),
        Ty::Infer { .. } => "_".into(),
        Ty::Ptr { ptr_token, ty } => format!("{}[{}]", ptr_token.span().as_str(), ty_to_string(ty.inner.as_ref())),
        Ty::Slice { slice_token, ty } => format!("{}[{}]", slice_token.span().as_str(), ty_to_string(ty.inner.as_ref())),
        
        Ty::Ref { ampersand_token, mut_token, ty } => format!(
            "{}{} {}",
            ampersand_token.span().as_str(),
            if let Some(x) = mut_token.as_ref() {
                x.span().as_str().to_string()
            } else {
                String::new()
            },
            ty_to_string(ty.as_ref()),
        ),

        Ty::Never { .. } => "!".into(),
    }
}

pub fn path_type_to_string(path_type: &PathType) -> String {
    let mut result = String::new();

    if let Some(root) = path_type.root_opt.as_ref() {
        if let Some(qualified_root_path) = root.0.as_ref() {
            result.push('<');
            result.push_str(ty_to_string(qualified_root_path.inner.ty.as_ref()).as_str());
            if let Some(as_trait) = qualified_root_path.inner.as_trait.as_ref() {
                result.push_str(" as ");
                result.push_str(path_type_to_string(as_trait.1.as_ref()).as_str());
            }
            result.push('>');
        }

        result.push_str("::");
    }

    result.push_str(path_type_segment_to_string(&path_type.prefix).as_str());

    for suffix in path_type.suffix.iter() {
        result.push_str("::");
        result.push_str(&path_type_segment_to_string(&suffix.1).as_str());
    }

    result
}

pub fn path_type_segment_to_string(path_type_segment: &PathTypeSegment) -> String {
    let mut result = String::new();

    result.push_str(path_type_segment.name.as_str());

    if let Some(generics) = path_type_segment.generics_opt.as_ref() {
        if generics.0.is_some() {
            result.push_str("::");
        }
        result.push_str(generic_args_to_string(&generics.1).as_str());
    }
    
    result
}

pub fn path_expr_to_string(path_expr: &PathExpr) -> String {
    let mut result = String::new();

    if let Some(root) = path_expr.root_opt.as_ref() {
        if let Some(qualified_root_path) = root.0.as_ref() {
            result.push('<');
            result.push_str(ty_to_string(qualified_root_path.inner.ty.as_ref()).as_str());
            if let Some(as_trait) = qualified_root_path.inner.as_trait.as_ref() {
                result.push_str(" as ");
                result.push_str(path_type_to_string(as_trait.1.as_ref()).as_str());
            }
            result.push('>');
        }

        result.push_str("::");
    }

    result.push_str(path_expr_segment_to_string(&path_expr.prefix).as_str());

    for suffix in path_expr.suffix.iter() {
        result.push_str("::");
        result.push_str(&path_expr_segment_to_string(&suffix.1).as_str());
    }

    result
}

pub fn path_expr_segment_to_string(path_expr_segment: &PathExprSegment) -> String {
    let mut result = String::new();

    result.push_str(path_expr_segment.name.as_str());

    if let Some(generics) = path_expr_segment.generics_opt.as_ref() {
        result.push_str("::");
        result.push_str(generic_args_to_string(&generics.1).as_str());
    }
    
    result
}

pub fn generic_args_to_string(generic_args: &GenericArgs) -> String {
    let mut result = String::new();

    result.push('<');

    for parameter in generic_args.parameters.inner.value_separator_pairs.iter() {
        result.push_str(ty_to_string(&parameter.0).as_str());
        result.push_str(", ");
    }

    if let Some(parameter) = generic_args.parameters.inner.final_value_opt.as_ref() {
        result.push_str(ty_to_string(parameter.as_ref()).as_str());
    }

    result.push('>');
    result
}

pub fn ty_tuple_descriptor_to_string(ty_tuple_descriptor: &TyTupleDescriptor) -> String {
    let mut result = String::new();

    result.push('(');

    if let TyTupleDescriptor::Cons { head, tail, .. } = ty_tuple_descriptor {
        result.push_str(ty_to_string(head.as_ref()).as_str());
        result.push_str(", ");
        for ty in tail {
            result.push_str(ty_to_string(ty).as_str());
        }
    }

    result.push(')');
    result
}

pub fn ty_array_descriptor_to_string(ty_array_descriptor: &TyArrayDescriptor) -> String {
    let mut result = String::new();
    result.push('[');
    result.push_str(ty_to_string(&ty_array_descriptor.ty).as_str());
    result.push_str("; ");
    result.push_str(ty_array_descriptor.length.span().as_str());
    result.push(']');
    result
}

#[inline]
pub fn path_expr_to_path_type(path_expr: &PathExpr) -> PathType {
    PathType {
        root_opt: path_expr.root_opt.clone(),
        
        prefix: PathTypeSegment {
            name: path_expr.prefix.name.clone(),
            generics_opt: path_expr.prefix.generics_opt.as_ref()
                .map(|(c, x)| (Some(c.clone()), x.clone())),
        },

        suffix: path_expr.suffix.iter()
            .map(|(c, s)| {
                (
                    c.clone(),
                    PathTypeSegment {
                        name: s.name.clone(),
                        generics_opt: s.generics_opt.as_ref()
                            .map(|(c, x)| (Some(c.clone()), x.clone())),
                    }
                )
            })
            .collect(),
    }
}

#[inline]
pub fn path_type_to_path_expr(path_type: &PathType) -> PathExpr {
    PathExpr {
        root_opt: path_type.root_opt.clone(),

        prefix: PathExprSegment {
            name: path_type.prefix.name.clone(),
            generics_opt: path_type.prefix.generics_opt.as_ref()
                .map(|(_, x)| (DoubleColonToken::default(), x.clone())),
        },

        suffix: path_type.suffix.iter()
            .map(|(c, s)| {
                (
                    c.clone(),
                    PathExprSegment {
                        name: s.name.clone(),
                        generics_opt: s.generics_opt.as_ref()
                            .map(|(_, x)| (DoubleColonToken::default(), x.clone())),
                    }
                )
            })
            .collect(),

        incomplete_suffix: false,
    }
}

#[inline]
pub fn empty_tuple_ty() -> Ty {
    Ty::Tuple(Parens {
        inner: TyTupleDescriptor::Nil,
        span: Span::dummy(),
    })
}

#[inline]
pub fn create_ident_ty(name: &str) -> Ty {
    Ty::Path(PathType {
        root_opt: None,
        prefix: PathTypeSegment {
            name: BaseIdent::new_no_span(name.into()),
            generics_opt: None,
        },
        suffix: vec![],
    })
}
