use sway_ast::{attribute::Attribute, *};
use sway_types::BaseIdent;

pub fn fold_expr_idents(expr: &Expr) -> Vec<BaseIdent> {
    let mut result = vec![];

    match expr {
        Expr::Path(PathExpr { prefix, .. }) => {
            result.push(prefix.name.clone());
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

pub fn check_attribute_decls(
    attribute_decls: &[AttributeDecl],
    attribute_name: &str,
    attribute_arg_names: &[&str],
) -> bool {
    for attribute_decl in attribute_decls {
        for attribute in attribute_decl.attribute.inner.value_separator_pairs.iter() {
            if check_attribute(&attribute.0, attribute_name, attribute_arg_names) {
                return true;
            }
        }

        if let Some(attribute) = attribute_decl.attribute.inner.final_value_opt.as_ref() {
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

            for attribute_arg in args.inner.value_separator_pairs.iter() {
                if attribute_arg.0.name.as_str() == attribute_arg_name {
                    result = true;
                    break;
                }
            }

            if let Some(attribute_arg) = args.inner.final_value_opt.as_ref() {
                if attribute_arg.name.as_str() == attribute_arg_name {
                    result = true;
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

    let variable_idents = fold_expr_idents(args.inner.final_value_opt.as_ref().unwrap());

    // TODO: need to support paths with multiple idents
    if variable_idents.len() != 1 {
        return None;
    }

    Some((storage_idents[1].clone(), variable_idents[0].clone()))
}
