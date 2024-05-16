use std::{cell::RefCell, rc::Rc};
use sway_ast::{
    keywords::{CloseAngleBracketToken, Keyword, OpenAngleBracketToken, StrToken, Token}, ty::TyTupleDescriptor, AngleBrackets, CommaToken, DoubleColonToken, Expr, ExprArrayDescriptor, ExprTupleDescriptor, FnSignature, GenericArgs, Literal, MatchBranchKind, Parens, PathExpr, PathType, PathTypeSegment, Punctuated, Ty
};
use sway_types::{BaseIdent, Span};

use crate::project::Project;

#[derive(Clone, Debug, PartialEq)]
pub enum AstVariableKind {
    Constant,
    Storage,
    Configurable,
    Parameter,
    Local,
}

#[derive(Clone, Debug)]
pub struct AstVariable {
    pub kind: AstVariableKind,
    pub name: String,
    pub ty: Ty,
}

#[derive(Debug, Default)]
pub struct AstScope {
    pub parent: Option<Rc<RefCell<AstScope>>>,
    pub variables: Vec<Rc<RefCell<AstVariable>>>,
    pub functions: Vec<Rc<RefCell<FnSignature>>>,
}

impl AstScope {
    pub fn get_variable(&self, name: &str, is_storage: bool) -> Option<Rc<RefCell<AstVariable>>> {
        for variable in self.variables.iter().rev() {
            if (variable.borrow().kind == AstVariableKind::Storage) != is_storage {
                continue;
            }

            if variable.borrow().name == name {
                return Some(variable.clone());
            }
        }

        if let Some(parent) = self.parent.as_ref() {
            if let Some(variable) = parent.borrow().get_variable(name, is_storage) {
                return Some(variable.clone());
            }
        }

        None
    }

    pub fn get_expr_ty(&self, expr: &Expr, project: &mut Project) -> Ty {
        match expr {
            Expr::Error(_, _) => todo!("{expr:#?}"),

            Expr::Path(path) => {
                // Check if the path is a single identifier and look it up as a variable
                if path.root_opt.is_none() && path.suffix.is_empty() {
                    println!("variables: {:#?}", self.variables);
                    if let Some(variable) = self.get_variable(path.prefix.name.as_str(), false) {
                        return variable.borrow().ty.clone();
                    }
                }

                todo!("{expr:#?}")
            }

            Expr::Literal(literal) => match literal {
                Literal::String(_) => Ty::StringSlice(StrToken::new(Span::dummy())),
                Literal::Char(_) => todo!("char does not have a type name in sway yet"),
                Literal::Int(_) => Ty::Path(PathType {
                    root_opt: None,
                    prefix: PathTypeSegment {
                        name: BaseIdent::new_no_span("u64".into()),
                        generics_opt: None,
                    },
                    suffix: vec![],
                }),
                Literal::Bool(_) => Ty::Path(PathType {
                    root_opt: None,
                    prefix: PathTypeSegment {
                        name: BaseIdent::new_no_span("bool".into()),
                        generics_opt: None,
                    },
                    suffix: vec![],
                }),
            }

            Expr::AbiCast { args, .. } => Ty::Path(args.inner.name.clone()),

            Expr::Struct { path, fields } => {
                //
                // TODO:
                // 1. Resolve type using both `path` and `fields`
                // 2. Resolve and return full type path (i.e: StorageKey<T> => std::storage::storage_key::StorageKey<T>)
                //

                Ty::Path(PathType {
                    root_opt: path.root_opt.clone(),
                    prefix: PathTypeSegment {
                        name: path.prefix.name.clone(),
                        generics_opt: path.prefix.generics_opt.clone().map(|(t, g)| (Some(t), g)),
                    },
                    suffix: vec![],
                })
            }

            Expr::Tuple(tuple) => {
                match &tuple.inner {
                    ExprTupleDescriptor::Nil => {
                        Ty::Tuple(Parens {
                            inner: TyTupleDescriptor::Nil,
                            span: Span::dummy(),
                        })
                    }

                    ExprTupleDescriptor::Cons { head, tail, .. } => {
                        let mut value_separator_pairs = vec![];
    
                        for expr in tail {
                            value_separator_pairs.push((self.get_expr_ty(expr, project), CommaToken::new(Span::dummy())));
                        }
    
                        let final_value_opt = value_separator_pairs.pop().map(|x| Box::new(x.0));
    
                        Ty::Tuple(Parens {
                            inner: TyTupleDescriptor::Cons {
                                head: Box::new(self.get_expr_ty(head, project)),
                                comma_token: CommaToken::new(Span::dummy()),
                                tail: Punctuated {
                                    value_separator_pairs,
                                    final_value_opt,
                                },
                            },
                            span: Span::dummy(),
                        })
                    }
                }
            }

            Expr::Parens(parens) => self.get_expr_ty(parens.inner.as_ref(), project),

            Expr::Block(block) => {
                if let Some(expr) = block.inner.final_expr_opt.as_ref() {
                    return self.get_expr_ty(expr, project);
                }

                Ty::Tuple(Parens {
                    inner: TyTupleDescriptor::Nil,
                    span: Span::dummy(),
                })
            }

            Expr::Array(array) => {
                match &array.inner {
                    ExprArrayDescriptor::Sequence(sequence) => {
                        if let Some((expr, _)) = sequence.value_separator_pairs.first() {
                            self.get_expr_ty(expr, project)
                        } else if let Some(expr) = sequence.final_value_opt.as_ref() {
                            self.get_expr_ty(expr, project)
                        } else {
                            Ty::Tuple(Parens {
                                inner: TyTupleDescriptor::Nil,
                                span: Span::dummy(),
                            })
                        }
                    }

                    ExprArrayDescriptor::Repeat { value, .. } => self.get_expr_ty(value, project),
                }
            }

            Expr::Asm(_) => {
                //
                // TODO: Get the type of the return value from the asm block if any
                //

                Ty::Tuple(Parens {
                    inner: TyTupleDescriptor::Nil,
                    span: Span::dummy(),
                })
            }

            Expr::Return { .. } => {
                Ty::Tuple(Parens {
                    inner: TyTupleDescriptor::Nil,
                    span: Span::dummy(),
                })
            }

            Expr::If(if_expr) => {
                if let Some(expr) = if_expr.then_block.inner.final_expr_opt.as_ref() {
                    return self.get_expr_ty(expr, project);
                }

                Ty::Tuple(Parens {
                    inner: TyTupleDescriptor::Nil,
                    span: Span::dummy(),
                })
            }

            Expr::Match { branches, .. } => {
                if let Some(branch) = branches.inner.first() {
                    match &branch.kind {
                        MatchBranchKind::Block { block, .. } => {
                            if let Some(expr) = block.inner.final_expr_opt.as_ref() {
                                return self.get_expr_ty(expr, project);
                            }
            
                            return Ty::Tuple(Parens {
                                inner: TyTupleDescriptor::Nil,
                                span: Span::dummy(),
                            });
                        }

                        MatchBranchKind::Expr { expr, .. } => {
                            return self.get_expr_ty(expr, project);
                        }
                    }
                }

                Ty::Tuple(Parens {
                    inner: TyTupleDescriptor::Nil,
                    span: Span::dummy(),
                })
            }

            Expr::While { .. } => Ty::Tuple(Parens {
                inner: TyTupleDescriptor::Nil,
                span: Span::dummy(),
            }),

            Expr::For { .. } => Ty::Tuple(Parens {
                inner: TyTupleDescriptor::Nil,
                span: Span::dummy(),
            }),

            Expr::FuncApp { func, args } => {
                let func_type = self.get_expr_ty(func, project);
                todo!("{expr:#?}")
            }

            Expr::Index { target, .. } => {
                let target_type = self.get_expr_ty(target, project);

                let Ty::Array(target_type) = target_type else {
                    panic!("Expected array type, got: {target_type:#?}");
                };

                target_type.inner.ty.as_ref().clone()
            }

            Expr::MethodCall { target, path_seg, contract_args_opt, args, .. } => {
                let target_type = self.get_expr_ty(target, project);

                let resolver = project.resolver.borrow();
                let resolved = resolver.resolve_ty(&target_type);
                println!("{resolved:#?}");
                
                todo!("{expr:#?}")
            }

            Expr::FieldProjection { target, name, .. } => {
                // Check if the field projection refers to a storage field and return a `core::storage::StorageKey<T>` type
                if let Expr::Path(PathExpr { root_opt, prefix, suffix, .. }) = target.as_ref() {
                    if root_opt.is_none() && prefix.name.as_str() == "storage" && suffix.is_empty() {
                        let variable = self.get_variable(name.as_str(), true).unwrap();

                        return Ty::Path(PathType {
                            root_opt: None,
                            prefix: PathTypeSegment {
                                name: BaseIdent::new_no_span("core".into()),
                                generics_opt: None,
                            },
                            suffix: vec![
                                (DoubleColonToken::new(Span::dummy()), PathTypeSegment {
                                    name: BaseIdent::new_no_span("storage".into()),
                                    generics_opt: None,
                                }),
                                (DoubleColonToken::new(Span::dummy()), PathTypeSegment {
                                    name: BaseIdent::new_no_span("StorageKey".into()),
                                    generics_opt: Some((None, GenericArgs {
                                        parameters: AngleBrackets {
                                            open_angle_bracket_token: OpenAngleBracketToken::new(Span::dummy()),
                                            inner: Punctuated {
                                                value_separator_pairs: vec![],
                                                final_value_opt: Some(Box::new(variable.borrow().ty.clone())),
                                            },
                                            close_angle_bracket_token: CloseAngleBracketToken::new(Span::dummy()),
                                        },
                                    })),
                                }),
                            ],
                        });
                    }
                }

                let target_type = self.get_expr_ty(target, project);
                
                let resolver = project.resolver.borrow();
                let resolved = resolver.resolve_ty(&target_type);
                
                let Some(sway_ast::ItemKind::Struct(item_struct)) = resolved else {
                    panic!("Expected struct, found: {resolved:#?}")
                };

                let mut fields = vec![];

                for field in &item_struct.fields.inner {
                    fields.push(field);
                }

                let Some(field) = fields.iter().find(|f| f.value.name == *name) else {
                    todo!("{expr:#?}")
                };

                field.value.ty.clone()
            }

            Expr::TupleFieldProjection { target, field, .. } => {
                let target_type = self.get_expr_ty(target, project);

                let Ty::Tuple(target_type) = target_type else {
                    panic!("Expected tuple type, got: {target_type:#?}");
                };

                match &target_type.inner {
                    TyTupleDescriptor::Nil => panic!("Field access on empty tuple: {expr:#?}"),
                    
                    TyTupleDescriptor::Cons { head, tail, .. } => {
                        let index: usize = field.try_into().unwrap();
                        
                        if index == 0 {
                            return head.as_ref().clone();
                        }

                        let mut remaining = vec![];

                        for ty in tail {
                            remaining.push(ty);
                        }

                        remaining[index - 1].clone()
                    }
                }
            }

            Expr::Ref { expr, .. } => self.get_expr_ty(expr, project),
            Expr::Deref { expr, .. } => self.get_expr_ty(expr, project),
            
            Expr::Not { expr, .. } => self.get_expr_ty(expr, project),

            Expr::Mul { lhs, .. } |
            Expr::Div { lhs, .. } |
            Expr::Pow { lhs, .. } |
            Expr::Modulo { lhs, .. } |
            Expr::Add { lhs, .. } |
            Expr::Sub { lhs, .. } |
            Expr::Shl { lhs, .. } |
            Expr::Shr { lhs, .. } |
            Expr::BitAnd { lhs, .. } |
            Expr::BitXor { lhs, .. } |
            Expr::BitOr { lhs, .. } => {
                self.get_expr_ty(lhs, project)
            }

            Expr::Equal { .. } |
            Expr::NotEqual { .. } |
            Expr::LessThan { .. } |
            Expr::GreaterThan { .. } |
            Expr::LessThanEq { .. } |
            Expr::GreaterThanEq { .. } |
            Expr::LogicalAnd { .. } |
            Expr::LogicalOr { .. } => {
                Ty::Path(PathType {
                    root_opt: None,
                    prefix: PathTypeSegment {
                        name: BaseIdent::new_no_span("bool".into()),
                        generics_opt: None,
                    },
                    suffix: vec![],
                })
            }

            Expr::Reassignment { .. } => Ty::Tuple(Parens {
                inner: TyTupleDescriptor::Nil,
                span: Span::dummy(),
            }),

            Expr::Break { .. } => Ty::Tuple(Parens {
                inner: TyTupleDescriptor::Nil,
                span: Span::dummy(),
            }),

            Expr::Continue { .. } => Ty::Tuple(Parens {
                inner: TyTupleDescriptor::Nil,
                span: Span::dummy(),
            }),
        }
    }
}
