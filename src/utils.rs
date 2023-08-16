use sway_ast::{attribute::Attribute, *};
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

    let ("write" | "insert") = storage_idents.last().unwrap().as_str() else { return None };

    let args = fold_punctuated(&args.inner);
    let Some(arg) = args.last() else { return None };
    let variable_idents = fold_expr_idents(arg);

    // TODO: need to support paths with multiple idents
    if variable_idents.len() != 1 {
        return None;
    }

    Some((storage_idents[1].clone(), variable_idents[0].clone()))
}
