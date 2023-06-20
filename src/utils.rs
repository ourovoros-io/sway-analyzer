use sway_ast::{attribute::Attribute, *};
use sway_types::BaseIdent;

pub fn fold_expr_base_idents(expr: &Expr) -> Vec<BaseIdent> {
    let mut result = vec![];

    match expr {
        Expr::Path(PathExpr { prefix, .. }) => {
            result.push(prefix.name.clone());
        }

        Expr::Index { target, .. } => {
            result.extend(fold_expr_base_idents(target));
        }

        Expr::MethodCall { target, path_seg, .. } => {
            result.extend(fold_expr_base_idents(target));
            result.push(path_seg.name.clone());
        }

        Expr::FieldProjection { target, name, .. } => {
            result.extend(fold_expr_base_idents(target));
            result.push(name.clone());
        }

        Expr::TupleFieldProjection { target, .. } => {
            result.extend(fold_expr_base_idents(target));
        }

        _ => {}
    }

    result
}

pub fn fold_assignable_base_idents(assignable: &Assignable) -> Vec<BaseIdent> {
    let mut result = vec![];

    match assignable {
        Assignable::Var(ident) => {
            result.push(ident.clone());
        }

        Assignable::Index { target, .. } => {
            result.extend(fold_assignable_base_idents(target));
        }

        Assignable::FieldProjection { target, name, .. } => {
            result.extend(fold_assignable_base_idents(target));
            result.push(name.clone());
        }

        Assignable::TupleFieldProjection { target, .. } => {
            result.extend(fold_assignable_base_idents(target));
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
