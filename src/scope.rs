use crate::{project::Project, utils};
use std::{cell::RefCell, rc::Rc};
use sway_ast::{
    keywords::{CloseAngleBracketToken, Keyword, OpenAngleBracketToken, StrToken},
    ty::TyTupleDescriptor,
    AngleBrackets, Braces, CommaToken, DoubleColonToken, Expr, ExprArrayDescriptor,
    ExprTupleDescriptor, FnArg, FnArgs, FnSignature, GenericArgs, ItemAbi, ItemKind, ItemStruct,
    ItemTrait, ItemTraitItem, ItemTypeAlias, ItemUse, Literal, MatchBranchKind, Parens, PathExpr,
    PathExprSegment, PathType, PathTypeSegment, Pattern, PatternStructField, Punctuated, Traits,
    Ty, WhereBound, WhereClause,
};
use sway_types::{BaseIdent, Span, Spanned};

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
    parent: Option<Rc<RefCell<AstScope>>>,
    uses: Vec<Rc<RefCell<ItemUse>>>,
    variables: Vec<Rc<RefCell<AstVariable>>>,
    functions: Vec<Rc<RefCell<FnSignature>>>,
    structs: Vec<Rc<RefCell<ItemStruct>>>,
    abis: Vec<Rc<RefCell<ItemAbi>>>,
    traits: Vec<Rc<RefCell<ItemTrait>>>,
    type_aliases: Vec<Rc<RefCell<ItemTypeAlias>>>,
}

#[inline]
fn empty_tuple_ty() -> Ty {
    Ty::Tuple(Parens {
        inner: TyTupleDescriptor::Nil,
        span: Span::dummy(),
    })
}

impl AstScope {
    pub fn new(parent: Option<Rc<RefCell<AstScope>>>) -> Self {
        Self {
            parent,
            ..Default::default()
        }
    }

    #[inline]
    pub fn parent(&self) -> Option<Rc<RefCell<AstScope>>> {
        self.parent.clone()
    }

    #[inline]
    pub fn uses(&self) -> impl Iterator<Item = &Rc<RefCell<ItemUse>>> {
        self.uses.iter()
    }

    #[inline]
    pub fn find_use<F: Copy + FnMut(&&Rc<RefCell<ItemUse>>) -> bool>(&self, f: F) -> Option<Rc<RefCell<ItemUse>>> {
        if let Some(x) = self.uses.iter().find(f) {
            return Some(x.clone());
        }

        if let Some(parent) = self.parent.as_ref() {
            if let Some(x) = parent.borrow().find_use(f) {
                return Some(x.clone());
            }
        }

        None
    }
    
    #[inline]
    pub fn add_use(&mut self, item_use: &ItemUse) {
        //
        // TODO: ensure the use is not already declared
        //

        self.uses.push(Rc::new(RefCell::new(item_use.clone())));
    }

    #[inline]
    pub fn variables(&self) -> impl Iterator<Item = &Rc<RefCell<AstVariable>>> {
        self.variables.iter()
    }

    #[inline]
    pub fn add_variable(
        &mut self,
        project: &mut Project,
        kind: AstVariableKind,
        name: &BaseIdent,
        ty: &Ty,
    ) {
        self.variables.push(Rc::new(RefCell::new(AstVariable {
            kind,
            name: name.to_string(),
            ty: self.expand_ty(project, ty),
        })));
    }

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

    #[inline]
    pub fn functions(&self) -> impl Iterator<Item = &Rc<RefCell<FnSignature>>> {
        self.functions.iter()
    }

    #[inline]
    pub fn add_function(&mut self, project: &mut Project, fn_signature: &FnSignature) {
        self.functions.push(Rc::new(RefCell::new(self.expand_fn_signature(project, fn_signature))));
    }

    pub fn get_fn_signature(
        &self,
        project: &mut Project,
        fn_name: &PathExprSegment,
        args: &Parens<Punctuated<Expr, CommaToken>>,
    ) -> Option<&FnSignature> {
        //
        // TODO:
        //
        // We need to find the `fn` we are looking for.
        // We need to ensure the argument types of the `fn` match the types of the supplied `args`.
        //
        // If the `fn` is not defined in the current module, we need to find a `use` statement that imports a valid `fn`:
        // 1. Check `prelude` module of the `core` library
        // 2. Check `prelude` module of the `std` library
        // 3. Check all explicit `use` statements
        //
        // Once we find the `fn`, return the signature of the `fn`
        //

        todo!()
    }

    pub fn get_impl_fn_signature(
        &self,
        project: &mut Project,
        ty: &Ty,
        fn_name: &PathExprSegment,
        args: &Parens<Punctuated<Expr, CommaToken>>,
    ) -> Option<&FnSignature> {
        //
        // TODO:
        //
        // We need to find a valid `impl` that contains the `fn` we are looking for.
        // We need to ensure the argument types of the `fn` match the types of the supplied `args`.
        //
        // If the `impl` is not defined in the current module, we need to find a `use` statement that imports a valid `impl` containing the `fn`:
        // 1. Check `prelude` module of the `core` library
        // 2. Check `prelude` module of the `std` library
        // 3. Check all explicit `use` statements
        //
        // Once we find the `impl` containing the `fn`, return the signature of the `fn`
        //

        todo!("Get function signature for {}::{}{}", utils::ty_to_string(ty), fn_name.name.span().as_str(), args.span.as_str())
    }

    /// Gets an iterator over the current structs in the scope.
    #[inline]
    pub fn structs(&self) -> impl Iterator<Item = &Rc<RefCell<ItemStruct>>> {
        self.structs.iter()
    }

    #[inline]
    pub fn find_struct<F: Copy + FnMut(&&Rc<RefCell<ItemStruct>>) -> bool>(&self, f: F) -> Option<Rc<RefCell<ItemStruct>>> {
        if let Some(x) = self.structs.iter().find(f) {
            return Some(x.clone());
        }

        if let Some(parent) = self.parent.as_ref() {
            if let Some(x) = parent.borrow().find_struct(f) {
                return Some(x.clone());
            }
        }

        None
    }

    /// Adds a struct to the scope.
    #[inline]
    pub fn add_struct(&mut self, project: &mut Project, item_struct: &ItemStruct) {
        let mut item_struct = item_struct.clone();

        if let Some(where_clause) = item_struct.where_clause_opt.as_mut() {
            *where_clause = self.expand_where_clause(project, where_clause);
        }

        for (field, _) in item_struct.fields.inner.value_separator_pairs.iter_mut() {
            field.value.ty = self.expand_ty(project, &field.value.ty);
        }

        if let Some(field) = item_struct.fields.inner.final_value_opt.as_mut() {
            field.value.ty = self.expand_ty(project, &field.value.ty);
        }

        self.structs.push(Rc::new(RefCell::new(item_struct)));
    }

    #[inline]
    pub fn abis(&self) -> impl Iterator<Item = &Rc<RefCell<ItemAbi>>> {
        self.abis.iter()
    }

    #[inline]
    pub fn find_abi<F: Copy + FnMut(&&Rc<RefCell<ItemAbi>>) -> bool>(&self, f: F) -> Option<Rc<RefCell<ItemAbi>>> {
        if let Some(x) = self.abis.iter().find(f) {
            return Some(x.clone());
        }

        if let Some(parent) = self.parent.as_ref() {
            if let Some(x) = parent.borrow().find_abi(f) {
                return Some(x.clone());
            }
        }

        None
    }

    #[inline]
    pub fn add_abi(&mut self, project: &mut Project, item_abi: &ItemAbi) {
        let mut item_abi = item_abi.clone();

        if let Some(super_traits) = item_abi.super_traits.as_mut() {
            super_traits.1.prefix = self.expand_path_type(project, &super_traits.1.prefix);
            
            for (_, suffix) in super_traits.1.suffixes.iter_mut() {
                *suffix = self.expand_path_type(project, suffix);
            }
        }

        for item in item_abi.abi_items.inner.iter_mut() {
            match &mut item.value {
                ItemTraitItem::Fn(fn_signature, _) => {
                    fn_signature.arguments = Parens {
                        inner: match &fn_signature.arguments.inner {
                            FnArgs::Static(args) => {
                                let mut value_separator_pairs = vec![];
        
                                for arg in args {
                                    value_separator_pairs.push((
                                        FnArg {
                                            pattern: self.expand_pattern(project, &arg.pattern),
                                            colon_token: arg.colon_token.clone(),
                                            ty: self.expand_ty(project, &arg.ty),
                                        },
                                        CommaToken::default(),
                                    ));
                                }
        
                                let final_value_opt = value_separator_pairs.pop().map(|x| Box::new(x.0));
        
                                FnArgs::Static(Punctuated {
                                    value_separator_pairs,
                                    final_value_opt,
                                })
                            }
        
                            FnArgs::NonStatic {
                                self_token,
                                ref_self,
                                mutable_self,
                                args_opt,
                            } => FnArgs::NonStatic {
                                self_token: self_token.clone(),
                                ref_self: ref_self.clone(),
                                mutable_self: mutable_self.clone(),
                                args_opt: args_opt.as_ref().map(|(comma, args)| {
                                    let mut value_separator_pairs = vec![];
        
                                    for arg in args {
                                        value_separator_pairs.push((
                                            FnArg {
                                                pattern: self.expand_pattern(project, &arg.pattern),
                                                colon_token: arg.colon_token.clone(),
                                                ty: self.expand_ty(project, &arg.ty),
                                            },
                                            CommaToken::default(),
                                        ));
                                    }
        
                                    let final_value_opt = value_separator_pairs.pop().map(|x| Box::new(x.0));
        
                                    (
                                        comma.clone(),
                                        Punctuated {
                                            value_separator_pairs,
                                            final_value_opt,
                                        },
                                    )
                                }),
                            },
                        },
        
                        span: fn_signature.arguments.span.clone(),
                    };
        
                    fn_signature.return_type_opt = fn_signature.return_type_opt.as_ref()
                        .map(|(arrow, ty)| (arrow.clone(), self.expand_ty(project, ty)));
        
                    fn_signature.where_clause_opt = fn_signature.where_clause_opt.as_ref().map(|where_clause| self.expand_where_clause(project, where_clause));
                }

                ItemTraitItem::Const(item_const, _) => {
                    item_const.ty_opt = item_const.ty_opt.as_ref().map(|(c, ty)| (c.clone(), self.expand_ty(project, &ty)));
                }

                ItemTraitItem::Type(item_type, _) => {
                    item_type.ty_opt = item_type.ty_opt.as_ref().map(|ty| self.expand_ty(project, &ty));
                }

                ItemTraitItem::Error(_, _) => panic!("Encountered an error while parsing Sway AST"),
            }
        }

        if let Some(abi_defs) = item_abi.abi_defs_opt.as_mut() {
            for item_fn in abi_defs.inner.iter_mut() {
                let fn_signature = &mut item_fn.value.fn_signature;

                fn_signature.arguments = Parens {
                    inner: match &fn_signature.arguments.inner {
                        FnArgs::Static(args) => {
                            let mut value_separator_pairs = vec![];
    
                            for arg in args {
                                value_separator_pairs.push((
                                    FnArg {
                                        pattern: self.expand_pattern(project, &arg.pattern),
                                        colon_token: arg.colon_token.clone(),
                                        ty: self.expand_ty(project, &arg.ty),
                                    },
                                    CommaToken::default(),
                                ));
                            }
    
                            let final_value_opt = value_separator_pairs.pop().map(|x| Box::new(x.0));
    
                            FnArgs::Static(Punctuated {
                                value_separator_pairs,
                                final_value_opt,
                            })
                        }
    
                        FnArgs::NonStatic {
                            self_token,
                            ref_self,
                            mutable_self,
                            args_opt,
                        } => FnArgs::NonStatic {
                            self_token: self_token.clone(),
                            ref_self: ref_self.clone(),
                            mutable_self: mutable_self.clone(),
                            args_opt: args_opt.as_ref().map(|(comma, args)| {
                                let mut value_separator_pairs = vec![];
    
                                for arg in args {
                                    value_separator_pairs.push((
                                        FnArg {
                                            pattern: self.expand_pattern(project, &arg.pattern),
                                            colon_token: arg.colon_token.clone(),
                                            ty: self.expand_ty(project, &arg.ty),
                                        },
                                        CommaToken::default(),
                                    ));
                                }
    
                                let final_value_opt = value_separator_pairs.pop().map(|x| Box::new(x.0));
    
                                (
                                    comma.clone(),
                                    Punctuated {
                                        value_separator_pairs,
                                        final_value_opt,
                                    },
                                )
                            }),
                        },
                    },
    
                    span: fn_signature.arguments.span.clone(),
                };
    
                fn_signature.return_type_opt = fn_signature.return_type_opt.as_ref()
                    .map(|(arrow, ty)| (arrow.clone(), self.expand_ty(project, ty)));
    
                fn_signature.where_clause_opt = fn_signature.where_clause_opt.as_ref().map(|where_clause| self.expand_where_clause(project, where_clause));

                item_fn.value.body.inner.statements.clear();
                item_fn.value.body.inner.final_expr_opt = None;
            }
        }

        self.abis.push(Rc::new(RefCell::new(item_abi)));
    }

    #[inline]
    pub fn traits(&self) -> impl Iterator<Item = &Rc<RefCell<ItemTrait>>> {
        self.traits.iter()
    }

    #[inline]
    pub fn find_trait<F: Copy + FnMut(&&Rc<RefCell<ItemTrait>>) -> bool>(&self, f: F) -> Option<Rc<RefCell<ItemTrait>>> {
        if let Some(x) = self.traits.iter().find(f) {
            return Some(x.clone());
        }

        if let Some(parent) = self.parent.as_ref() {
            if let Some(x) = parent.borrow().find_trait(f) {
                return Some(x.clone());
            }
        }

        None
    }

    #[inline]
    pub fn add_trait(&mut self, project: &mut Project, item_trait: &ItemTrait) {
        let mut item_trait = item_trait.clone();
        
        if let Some(where_clause) = item_trait.where_clause_opt.as_mut() {
            *where_clause = self.expand_where_clause(project, where_clause);
        }

        if let Some(super_traits) = item_trait.super_traits.as_mut() {
            super_traits.1.prefix = self.expand_path_type(project, &super_traits.1.prefix);
            
            for (_, suffix) in super_traits.1.suffixes.iter_mut() {
                *suffix = self.expand_path_type(project, suffix);
            }
        }

        for item in item_trait.trait_items.inner.iter_mut() {
            match &mut item.value {
                ItemTraitItem::Fn(fn_signature, _) => {
                    *fn_signature = self.expand_fn_signature(project, fn_signature);
                }

                ItemTraitItem::Const(item_const, _) => {
                    if let Some((_, ty)) = item_const.ty_opt.as_mut() {
                        *ty = self.expand_ty(project, ty);
                    }
                }

                ItemTraitItem::Type(item_type, _) => {
                    if let Some(ty) = item_type.ty_opt.as_mut() {
                        *ty = self.expand_ty(project, ty);
                    }
                }

                ItemTraitItem::Error(_, _) => panic!("Encountered an error while parsing Sway AST"),
            }
        }

        if let Some(trait_defs) = item_trait.trait_defs_opt.as_mut() {
            for item_fn in trait_defs.inner.iter_mut() {
                item_fn.value.fn_signature = self.expand_fn_signature(project, &item_fn.value.fn_signature);
                item_fn.value.body.inner.statements.clear();
                item_fn.value.body.inner.final_expr_opt = None;
            }
        }

        self.traits.push(Rc::new(RefCell::new(item_trait)));
    }

    #[inline]
    pub fn type_aliases(&self) -> impl Iterator<Item = &Rc<RefCell<ItemTypeAlias>>> {
        self.type_aliases.iter()
    }

    #[inline]
    pub fn find_type_alias<F: Copy + FnMut(&&Rc<RefCell<ItemTypeAlias>>) -> bool>(&self, f: F) -> Option<Rc<RefCell<ItemTypeAlias>>> {
        if let Some(x) = self.type_aliases.iter().find(f) {
            return Some(x.clone());
        }

        if let Some(parent) = self.parent.as_ref() {
            if let Some(x) = parent.borrow().find_type_alias(f) {
                return Some(x.clone());
            }
        }

        None
    }

    #[inline]
    pub fn add_type_alias(&mut self, project: &mut Project, item_type_alias: &ItemTypeAlias) {
        let mut item_type_alias = item_type_alias.clone();
        
        item_type_alias.ty = self.expand_ty(project, &item_type_alias.ty);

        self.type_aliases.push(Rc::new(RefCell::new(item_type_alias)));
    }

    pub fn get_expr_ty(&self, expr: &Expr, project: &mut Project) -> Ty {
        match expr {
            Expr::Error(_, _) => todo!("{expr:#?}"),

            Expr::Path(path) => {
                // Check if the path is a single identifier and look it up as a variable
                if path.root_opt.is_none() && path.suffix.is_empty() {
                    if let Some(variable) = self.get_variable(path.prefix.name.as_str(), false) {
                        return variable.borrow().ty.clone();
                    }
                }

                todo!("{expr:#?}")
            }

            Expr::Literal(literal) => match literal {
                Literal::String(_) => Ty::StringSlice(StrToken::new(Span::dummy())),

                Literal::Char(_) => Ty::Path(PathType {
                    root_opt: None,
                    prefix: PathTypeSegment {
                        name: BaseIdent::new_no_span("char".into()),
                        generics_opt: None,
                    },
                    suffix: vec![],
                }),

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
            },

            Expr::AbiCast { args, .. } => Ty::Path(args.inner.name.clone()),

            Expr::Struct { path, fields } => {
                let path_expr = self.expand_path_expr(project, path);
                let path_type = utils::path_expr_to_path_type(&path_expr);
                Ty::Path(path_type)
            }

            Expr::Tuple(tuple) => match &tuple.inner {
                ExprTupleDescriptor::Nil => empty_tuple_ty(),

                ExprTupleDescriptor::Cons { head, tail, .. } => {
                    let mut value_separator_pairs = vec![];

                    for expr in tail {
                        value_separator_pairs.push((
                            self.get_expr_ty(expr, project),
                            CommaToken::default(),
                        ));
                    }

                    let final_value_opt = value_separator_pairs.pop().map(|x| Box::new(x.0));

                    Ty::Tuple(Parens {
                        inner: TyTupleDescriptor::Cons {
                            head: Box::new(self.get_expr_ty(head, project)),
                            comma_token: CommaToken::default(),
                            tail: Punctuated {
                                value_separator_pairs,
                                final_value_opt,
                            },
                        },
                        span: Span::dummy(),
                    })
                }
            },

            Expr::Parens(parens) => self.get_expr_ty(parens.inner.as_ref(), project),

            Expr::Block(block) => match block.inner.final_expr_opt.as_ref() {
                Some(expr) => self.get_expr_ty(expr, project),
                None => empty_tuple_ty(),
            },

            Expr::Array(array) => match &array.inner {
                ExprArrayDescriptor::Sequence(sequence) => {
                    if let Some((expr, _)) = sequence.value_separator_pairs.first() {
                        self.get_expr_ty(expr, project)
                    } else if let Some(expr) = sequence.final_value_opt.as_ref() {
                        self.get_expr_ty(expr, project)
                    } else {
                        empty_tuple_ty()
                    }
                }

                ExprArrayDescriptor::Repeat { value, .. } => self.get_expr_ty(value, project),
            },

            Expr::Asm(_) => {
                //
                // TODO: Get the type of the return value from the asm block if any
                //

                empty_tuple_ty()
            }

            Expr::Return { .. } => empty_tuple_ty(),

            Expr::If(if_expr) => {
                if let Some(expr) = if_expr.then_block.inner.final_expr_opt.as_ref() {
                    return self.get_expr_ty(expr, project);
                }

                empty_tuple_ty()
            }

            Expr::Match { branches, .. } => {
                if let Some(branch) = branches.inner.first() {
                    match &branch.kind {
                        MatchBranchKind::Block { block, .. } => {
                            if let Some(expr) = block.inner.final_expr_opt.as_ref() {
                                return self.get_expr_ty(expr, project);
                            }

                            return empty_tuple_ty();
                        }

                        MatchBranchKind::Expr { expr, .. } => {
                            return self.get_expr_ty(expr, project);
                        }
                    }
                }

                empty_tuple_ty()
            }

            Expr::While { .. } | Expr::For { .. } => empty_tuple_ty(),

            Expr::FuncApp { func, args } => todo!("{expr:#?}"),

            Expr::Index { target, .. } => {
                let target_type = self.get_expr_ty(target, project);

                let Ty::Array(target_type) = target_type else {
                    panic!("Expected array type, got: {target_type:#?}");
                };

                target_type.inner.ty.as_ref().clone()
            }

            Expr::MethodCall { target, path_seg, args, .. } => {
                let target_type = self.get_expr_ty(target, project);
                let fn_signature = self.get_impl_fn_signature(project, &target_type, path_seg, args).unwrap();

                let ty = fn_signature.return_type_opt.as_ref()
                    .map(|(_, ty)| ty.clone())
                    .unwrap_or_else(empty_tuple_ty);

                self.expand_ty(project, &ty)
            }

            Expr::FieldProjection { target, name, .. } => {
                // Check if the field projection refers to a storage field and return a `core::storage::StorageKey<T>` type
                if let Expr::Path(PathExpr { root_opt, prefix, suffix, .. }) = target.as_ref() {
                    if root_opt.is_none() && prefix.name.as_str() == "storage" && suffix.is_empty() {
                        let variable = self.get_variable(name.as_str(), true).unwrap();
                        let ty = self.expand_ty(project, &variable.borrow().ty);

                        return Ty::Path(PathType {
                            root_opt: None,
                            prefix: PathTypeSegment {
                                name: BaseIdent::new_no_span("core".into()),
                                generics_opt: None,
                            },
                            suffix: vec![
                                (
                                    DoubleColonToken::default(),
                                    PathTypeSegment {
                                        name: BaseIdent::new_no_span("storage".into()),
                                        generics_opt: None,
                                    },
                                ),
                                (
                                    DoubleColonToken::default(),
                                    PathTypeSegment {
                                        name: BaseIdent::new_no_span("StorageKey".into()),
                                        generics_opt: Some((
                                            Some(DoubleColonToken::default()),
                                            GenericArgs {
                                                parameters: AngleBrackets {
                                                    open_angle_bracket_token:
                                                        OpenAngleBracketToken::default(),
                                                    inner: Punctuated {
                                                        value_separator_pairs: vec![],
                                                        final_value_opt: Some(Box::new(ty)),
                                                    },
                                                    close_angle_bracket_token:
                                                        CloseAngleBracketToken::default(),
                                                },
                                            },
                                        )),
                                    },
                                ),
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

            Expr::Mul { lhs, .. }
            | Expr::Div { lhs, .. }
            | Expr::Pow { lhs, .. }
            | Expr::Modulo { lhs, .. }
            | Expr::Add { lhs, .. }
            | Expr::Sub { lhs, .. }
            | Expr::Shl { lhs, .. }
            | Expr::Shr { lhs, .. }
            | Expr::BitAnd { lhs, .. }
            | Expr::BitXor { lhs, .. }
            | Expr::BitOr { lhs, .. } => self.get_expr_ty(lhs, project),

            Expr::Equal { .. }
            | Expr::NotEqual { .. }
            | Expr::LessThan { .. }
            | Expr::GreaterThan { .. }
            | Expr::LessThanEq { .. }
            | Expr::GreaterThanEq { .. }
            | Expr::LogicalAnd { .. }
            | Expr::LogicalOr { .. } => Ty::Path(PathType {
                root_opt: None,
                prefix: PathTypeSegment {
                    name: BaseIdent::new_no_span("bool".into()),
                    generics_opt: None,
                },
                suffix: vec![],
            }),

            Expr::Reassignment { .. } => empty_tuple_ty(),

            Expr::Break { .. } | Expr::Continue { .. } => empty_tuple_ty(),
        }
    }

    #[inline]
    fn expand_path_type(&self, project: &mut Project, path_type: &PathType) -> PathType {
        utils::path_expr_to_path_type(
            &self.expand_path_expr(
                project,
                &utils::path_type_to_path_expr(path_type),
            ),
        )
    }

    fn expand_path_expr(&self, project: &mut Project, path_expr: &PathExpr) -> PathExpr {
        let resolver = project.resolver.clone();

        // Get the last part of the path expression
        let mut segment = match path_expr.suffix.last() {
            Some((_, suffix)) => suffix.clone(),
            None => path_expr.prefix.clone(),
        };

        // Count the number of generic parameters
        let mut input_generic_count = 0;
        
        if let Some((_, generics)) = segment.generics_opt.as_mut() {
            for _ in &generics.parameters.inner {
                input_generic_count += 1;
            }

            for (ty, _) in generics.parameters.inner.value_separator_pairs.iter_mut() {
                *ty = self.expand_ty(project, ty);
            }

            if let Some(ty) = generics.parameters.inner.final_value_opt.as_mut() {
                *ty = Box::new(self.expand_ty(project, ty.as_ref()));
            }
        }
        
        match path_expr.root_opt.as_ref() {
            Some(_) => {
                //
                // TODO: Find the module in the current project
                //

                todo!()
            }

            None => {
                //
                // Look for a type in the current module
                //
                
                if input_generic_count == 0 {
                    // 1. Check for a type alias in the current module
                    if let Some(type_alias) = self.find_type_alias(|item_type_alias| {
                        item_type_alias.borrow().name.as_str() == segment.name.as_str()
                    }) {
                        let type_alias = type_alias.borrow();

                        match &type_alias.ty {
                            Ty::Path(path_type) => {
                                return utils::path_type_to_path_expr(
                                    &self.expand_path_type(project, path_type),
                                );
                            }

                            _ => todo!("Handle non-path underlying type: {:#?}", type_alias),
                        }
                    }

                    // 2. Check for an abi in the current module
                    if let Some(item_abi) = self.find_abi(|item_abi| {
                        item_abi.borrow().name.as_str() == segment.name.as_str()
                    }) {
                        let item_abi = item_abi.borrow();

                        //
                        // TODO: find absolute path of current module and include it below
                        //
                        
                        return PathExpr {
                            root_opt: Some((None, DoubleColonToken::default())),
                            prefix: PathExprSegment {
                                name: item_abi.name.clone(),
                                generics_opt: None,
                            },
                            suffix: vec![],
                            incomplete_suffix: false,
                        };
                    }
                }

                // 3. Check for a struct in the current module
                if let Some(item_struct) = self.find_struct(|item_struct| {
                    let item_struct = item_struct.borrow();
                    
                    if item_struct.name.as_str() != segment.name.as_str() {
                        return false;
                    }
                    
                    item_struct.generics.as_ref().map(|x| {
                        let mut count = 0;
                        for _ in &x.parameters.inner {
                            count += 1;
                        }
                        count
                    }).unwrap_or(0) == input_generic_count
                }) {
                    let item_struct = item_struct.borrow();

                    //
                    // TODO: find absolute path of current module and include it below
                    //

                    return PathExpr {
                        root_opt: None,
                        prefix: PathExprSegment {
                            name: item_struct.name.clone(),
                            generics_opt: segment.generics_opt.clone(),
                        },
                        suffix: vec![],
                        incomplete_suffix: false,
                    };
                }

                // 4. Check for a trait in the current module
                if let Some(item_trait) = self.find_trait(|item_trait| {
                    let item_trait = item_trait.borrow();
                    
                    if item_trait.name.as_str() != segment.name.as_str() {
                        return false;
                    }

                    item_trait.generics.as_ref().map(|x| {
                        let mut count = 0;
                        for _ in &x.parameters.inner {
                            count += 1;
                        }
                        count
                    }).unwrap_or(0) == input_generic_count
                }) {
                    let item_trait = item_trait.borrow();

                    //
                    // TODO: find absolute path of current module and include it below
                    //
                    
                    return PathExpr {
                        root_opt: None,
                        prefix: PathExprSegment {
                            name: item_trait.name.clone(),
                            generics_opt: segment.generics_opt.clone(),
                        },
                        suffix: vec![],
                        incomplete_suffix: false,
                    };
                }
                
                // 5. Look an explicit `use` statement in the current module
                if let Some(item_use) = self.find_use(|item_use| {
                    let item_use = item_use.borrow();

                    for use_path_expr in utils::flatten_use_tree(None, &item_use.tree) {
                        let lhs = if let Some(suffix) = use_path_expr.suffix.last() {
                            &suffix.1
                        } else {
                            &use_path_expr.prefix
                        };
                        
                        if lhs.name.as_str() == segment.name.as_str() {
                            return true;
                        }
                    }

                    false
                }) {
                    let item_use = item_use.borrow();

                    for path_expr in utils::flatten_use_tree(None, &item_use.tree) {
                        if path_expr.suffix.last().map(|(_, s)| s.name.as_str() == segment.name.as_str()).unwrap_or(false) {
                            let mut expanded_path = path_expr.clone();

                            let expanded_segment = if let Some((_, segment)) = expanded_path.suffix.last_mut() {
                                segment
                            } else {
                                &mut expanded_path.prefix
                            };

                            *expanded_segment = segment.clone();
                            
                            return expanded_path;
                        }
                    }
                }

                let mut check_library_prelude = |library_name: &str| -> Option<PathExpr> {
                    let resolver = resolver.borrow();
                    let library = resolver.libraries.iter().find(|lib| lib.name == library_name)?;
                    
                    let Some(prelude) = library.modules.iter().find(|module| module.name.as_str() == "prelude") else {
                        panic!("Failed to find `{library_name}::prelude` module");
                    };

                    // 1. Check for a type alias or an abi defined in the prelude module
                    if input_generic_count == 0 {
                        for item in &prelude.inner.items {
                            let (
                                ItemKind::TypeAlias(ItemTypeAlias { name, .. })
                                | ItemKind::Abi(ItemAbi { name, .. })
                            ) = &item.value else {
                                continue;
                            };

                            if name.as_str() == segment.name.as_str() {
                                let mut expanded_path = self.expand_path_expr(project, path_expr);
                                let prefix = expanded_path.prefix.clone();

                                let expanded_segment = if let Some((_, segment)) = expanded_path.suffix.last_mut() {
                                    segment
                                } else {
                                    &mut expanded_path.prefix
                                };
    
                                *expanded_segment = segment.clone();
                                
                                expanded_path.prefix = PathExprSegment {
                                    name: BaseIdent::new_no_span(library_name.to_string()),
                                    generics_opt: None,
                                };

                                expanded_path.suffix.insert(0, (DoubleColonToken::default(), prefix));

                                return Some(expanded_path);
                            }
                        }
                    }

                    // 2. Check for a struct or trait defined in the prelude module
                    for item in &prelude.inner.items {
                        let (
                            ItemKind::Struct(ItemStruct { name, generics, .. })
                            | ItemKind::Trait(ItemTrait { name, generics, .. })
                        ) = &item.value else {
                            continue;
                        };

                        if name.as_str() != segment.name.as_str() {
                            continue;
                        }
    
                        if generics.as_ref().map(|x| {
                            let mut count = 0;
                            for _ in &x.parameters.inner {
                                count += 1;
                            }
                            count
                        }).unwrap_or(0) == input_generic_count {
                            let mut expanded_path = self.expand_path_expr(project, path_expr);
                            let prefix = expanded_path.prefix.clone();

                            let expanded_segment = if let Some((_, segment)) = expanded_path.suffix.last_mut() {
                                segment
                            } else {
                                &mut expanded_path.prefix
                            };

                            *expanded_segment = segment.clone();
                            
                            expanded_path.prefix = PathExprSegment {
                                name: BaseIdent::new_no_span(library_name.to_string()),
                                generics_opt: None,
                            };

                            expanded_path.suffix.insert(0, (DoubleColonToken::default(), prefix));

                            return Some(expanded_path);
                        }
                    }
                    
                    // 3. Check for an explicit use declared in the prelude module
                    for item in &prelude.inner.items {
                        let ItemKind::Use(item_use) = &item.value else {
                            continue;
                        };

                        for path_expr in utils::flatten_use_tree(None, &item_use.tree) {
                            if path_expr.suffix.last().map(|(_, s)| s.name.as_str() == segment.name.as_str()).unwrap_or(false) {
                                let mut expanded_path = path_expr.clone();

                                let expanded_segment = if let Some((_, segment)) = expanded_path.suffix.last_mut() {
                                    segment
                                } else {
                                    &mut expanded_path.prefix
                                };
    
                                *expanded_segment = segment.clone();
                                
                                if item_use.root_import.is_some() {
                                    let prefix = expanded_path.prefix.clone();

                                    expanded_path.prefix = PathExprSegment {
                                        name: BaseIdent::new_no_span(library_name.to_string()),
                                        generics_opt: None,
                                    };

                                    expanded_path.suffix.insert(0, (DoubleColonToken::default(), prefix));
                                }

                                return Some(expanded_path);
                            }
                        }
                    }

                    None
                };

                // 6. Check the std prelude
                if let Some(result) = check_library_prelude("std") {
                    return result;
                }

                // 7. Check the core prelude
                if let Some(result) = check_library_prelude("core") {
                    return result;
                }

                // 8. Check any available libraries
                for library in resolver.borrow().libraries.iter() {
                    if library.name == segment.name.as_str() {
                        return path_expr.clone();
                    }
                }

                // 9. Check for built-in types
                if path_expr.prefix.generics_opt.is_none() && path_expr.suffix.is_empty() {
                    if let "u8" | "u16" | "u32" | "u64" | "u256" | "bool" | "str" | "b256" = segment.name.as_str() {
                        return path_expr.clone();
                    }
                }

                panic!("Failed to expand path expression: {path_expr:#?}")
            }
        }
    }

    fn expand_ty(&self, project: &mut Project, ty: &Ty) -> Ty {
        let mut ty = ty.clone();

        if project.resolver.borrow().resolve_ty(&ty).is_some() {
            return ty;
        }

        match &mut ty {
            Ty::Path(path_type) => {
                *path_type = self.expand_path_type(project, path_type);
            }

            Ty::Tuple(tuple) => {
                if let TyTupleDescriptor::Cons { head, tail, .. } = &mut tuple.inner {
                    *head.as_mut() = self.expand_ty(project, head.as_ref());
                    
                    for (ty, _) in tail.value_separator_pairs.iter_mut() {
                        *ty = self.expand_ty(project, ty);
                    }

                    if let Some(ty) = tail.final_value_opt.as_mut() {
                        *ty.as_mut() = self.expand_ty(project, ty);
                    }
                }
            }

            Ty::Array(array) => {
                *array.inner.ty.as_mut() = self.expand_ty(project, array.inner.ty.as_ref());
            }

            Ty::Ptr { ty, .. } | Ty::Slice { ty, .. } => {
                *ty.inner.as_mut() = self.expand_ty(project, ty.inner.as_ref());
            }

            Ty::Ref { ty, .. } => {
                *ty.as_mut() = self.expand_ty(project, ty.as_ref());
            }

            _ => {}
        }

        ty
    }

    #[inline]
    fn expand_fn_signature(&self, project: &mut Project, fn_signature: &FnSignature) -> FnSignature {
        let mut fn_signature = fn_signature.clone();

        if let Some(args) = match &mut fn_signature.arguments.inner {
            FnArgs::Static(args) => Some(args),
            FnArgs::NonStatic { args_opt, .. } => args_opt.as_mut().map(|(_, args)| args),
        } {
            for (arg, _) in args.value_separator_pairs.iter_mut() {
                arg.pattern = self.expand_pattern(project, &arg.pattern);
            }

            if let Some(arg) = args.final_value_opt.as_mut() {
                arg.pattern = self.expand_pattern(project, &arg.pattern);
            }
        }

        if let Some((_, return_type)) = fn_signature.return_type_opt.as_mut() {
            *return_type = self.expand_ty(project, return_type);
        }

        if let Some(where_clause) = fn_signature.where_clause_opt.as_mut() {
            *where_clause = self.expand_where_clause(project, where_clause);
        }

        fn_signature
    }

    #[inline]
    fn expand_where_clause(&self, project: &mut Project, where_clause: &WhereClause) -> WhereClause {
        let mut where_clause = where_clause.clone();

        for (where_bound, _) in where_clause.bounds.value_separator_pairs.iter_mut() {
            where_bound.bounds.prefix = self.expand_path_type(project, &where_bound.bounds.prefix);

            for (_, suffix) in where_bound.bounds.suffixes.iter_mut() {
                *suffix = self.expand_path_type(project, suffix);
            }
        }

        if let Some(where_bound) = where_clause.bounds.final_value_opt.as_mut() {
            where_bound.bounds.prefix = self.expand_path_type(project, &where_bound.bounds.prefix);

            for (_, suffix) in where_bound.bounds.suffixes.iter_mut() {
                *suffix = self.expand_path_type(project, suffix);
            }
        }
        
        where_clause
    }

    fn expand_pattern(&self, project: &mut Project, pattern: &Pattern) -> Pattern {
        let mut pattern = pattern.clone();

        match &mut pattern {
            Pattern::Or { lhs, rhs, .. } => {
                *lhs.as_mut() = self.expand_pattern(project, lhs);
                *rhs.as_mut() = self.expand_pattern(project, rhs);
            },

            Pattern::Constant(path_expr) => {
                *path_expr = self.expand_path_expr(project, path_expr);
            }
            
            Pattern::Constructor { path, args } => {
                *path = self.expand_path_expr(project, path);

                for (arg, _) in args.inner.value_separator_pairs.iter_mut() {
                    *arg = self.expand_pattern(project, arg);
                }
                
                if let Some(arg) = args.inner.final_value_opt.as_mut() {
                    *arg.as_mut() = self.expand_pattern(project, arg);
                }
            }

            Pattern::Struct { path, fields } => {
                *path = self.expand_path_expr(project, path);
                
                for (field, _) in fields.inner.value_separator_pairs.iter_mut() {
                    if let PatternStructField::Field { pattern_opt: Some((_, pattern)), .. } = field {
                        *pattern.as_mut() = self.expand_pattern(project, pattern);
                    }
                }

                if let Some(field) = fields.inner.final_value_opt.as_mut() {
                    if let PatternStructField::Field { pattern_opt: Some((_, pattern)), .. } = field.as_mut() {
                        *pattern.as_mut() = self.expand_pattern(project, pattern);
                    }
                }
            },
            
            Pattern::Tuple(tuple) => {
                for (pattern, _) in tuple.inner.value_separator_pairs.iter_mut() {
                    *pattern = self.expand_pattern(project, pattern);
                }

                if let Some(pattern) = tuple.inner.final_value_opt.as_mut() {
                    *pattern.as_mut() = self.expand_pattern(project, pattern);
                }
            }

            _ => {}
        }
    
        pattern
    }
}
