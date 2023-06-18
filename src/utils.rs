use sway_ast::*;
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
