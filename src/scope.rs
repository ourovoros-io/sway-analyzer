use crate::{project::Project, utils::flatten_use_tree};
use std::{cell::RefCell, rc::Rc};
use sway_ast::{
    brackets::SquareBrackets, keywords::{CloseAngleBracketToken, Keyword, OpenAngleBracketToken, StarToken, StrToken, Token}, ty::{TyArrayDescriptor, TyTupleDescriptor}, AngleBrackets, Braces, CommaToken, DoubleColonToken, Expr, ExprArrayDescriptor, ExprTupleDescriptor, FnArg, FnArgs, FnSignature, GenericArgs, ItemAbi, ItemKind, ItemStruct, ItemTrait, ItemTypeAlias, ItemUse, Literal, MatchBranchKind, Parens, PathExpr, PathExprSegment, PathType, PathTypeSegment, Pattern, Punctuated, Traits, Ty, UseTree, WhereBound, WhereClause
};
use sway_types::{BaseIdent, Span};

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
    uses: Vec<ItemUse>,
    variables: Vec<Rc<RefCell<AstVariable>>>,
    functions: Vec<Rc<RefCell<FnSignature>>>,
    structs: Vec<ItemStruct>,
    abis: Vec<ItemAbi>,
    traits: Vec<ItemTrait>,
    types_aliases: Vec<ItemTypeAlias>,
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
    pub fn uses(&self) -> impl Iterator<Item = &ItemUse> {
        self.uses.iter()
    }

    pub fn add_use(&mut self, _project: &mut Project, item_use: &ItemUse) {
        //
        // TODO: ensure the use is not already declared
        //

        self.uses.push(item_use.clone());
        
    }

    #[inline]
    pub fn variables(&self) -> impl Iterator<Item = &Rc<RefCell<AstVariable>>> {
        self.variables.iter()
    }

    pub fn add_variable(&mut self, project: &mut Project, kind: AstVariableKind, name: &BaseIdent, ty: &Ty) {
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

    pub fn add_function(&mut self, project: &mut Project, fn_signature: &FnSignature) {
        self.functions.push(Rc::new(RefCell::new(FnSignature {
            visibility: fn_signature.visibility.clone(),
            fn_token: fn_signature.fn_token.clone(),
            name: fn_signature.name.clone(),
            generics: fn_signature.generics.clone(),

            arguments: Parens {
                inner: match &fn_signature.arguments.inner {
                    FnArgs::Static(args) => {
                        let mut value_separator_pairs = vec![];

                        for arg in args {
                            value_separator_pairs.push(
                                (
                                    FnArg {
                                        pattern: self.expand_pattern(project, &arg.pattern),
                                        colon_token: arg.colon_token.clone(),
                                        ty: self.expand_ty(project, &arg.ty),
                                    },
                                    CommaToken::new(Span::dummy())
                                )
                            );
                        }

                        let final_value_opt = value_separator_pairs.pop().map(|x| Box::new(x.0));

                        FnArgs::Static(Punctuated {
                            value_separator_pairs,
                            final_value_opt,
                        })
                    }

                    FnArgs::NonStatic { self_token, ref_self, mutable_self, args_opt } => FnArgs::NonStatic {
                        self_token: self_token.clone(),
                        ref_self: ref_self.clone(),
                        mutable_self: mutable_self.clone(),
                        args_opt: args_opt.as_ref().map(|(comma, args)| {
                            let mut value_separator_pairs = vec![];

                            for arg in args {
                                value_separator_pairs.push(
                                    (
                                        FnArg {
                                            pattern: self.expand_pattern(project, &arg.pattern),
                                            colon_token: arg.colon_token.clone(),
                                            ty: self.expand_ty(project, &arg.ty),
                                        },
                                        CommaToken::new(Span::dummy())
                                    )
                                );
                            }

                            let final_value_opt = value_separator_pairs.pop().map(|x| Box::new(x.0));

                            (
                                comma.clone(),
                                Punctuated {
                                    value_separator_pairs,
                                    final_value_opt,
                                }
                            )
                        }),
                    },
                },
                
                span: fn_signature.arguments.span.clone(),
            },

            return_type_opt: fn_signature.return_type_opt.as_ref().map(|(arrow, ty)| {
                (
                    arrow.clone(),
                    self.expand_ty(project, ty)
                )
            }),

            where_clause_opt: fn_signature.where_clause_opt.as_ref().map(|where_clause| {
                let mut value_separator_pairs = vec![];

                for where_bound in &where_clause.bounds {
                    value_separator_pairs.push(
                        (
                            WhereBound {
                                ty_name: where_bound.ty_name.clone(),
                                colon_token: where_bound.colon_token.clone(),
                                bounds: Traits {
                                    prefix: self.expand_path_type(project, &where_bound.bounds.prefix),
                                    suffixes: where_bound.bounds.suffixes.iter().map(|(add_token, path_type)| {
                                        (
                                            add_token.clone(),
                                            self.expand_path_type(project, path_type)
                                        )
                                    })
                                    .collect(),
                                },
                            },
                            CommaToken::new(Span::dummy())
                        )
                    );
                }

                let final_value_opt = value_separator_pairs.pop().map(|x| Box::new(x.0));

                WhereClause {
                    where_token: where_clause.where_token.clone(),
                    bounds: Punctuated {
                        value_separator_pairs,
                        final_value_opt,
                    },
                }
            }),
        })));
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

        todo!()
    }


    #[inline]
    pub fn structs(&self) -> impl Iterator<Item = &ItemStruct> {
        self.structs.iter()
    }

    pub fn add_struct(&mut self, project: &mut Project, item_struct: &ItemStruct) {
        self.structs.push(item_struct.clone());
    }

    #[inline]
    pub fn abis(&self) -> impl Iterator<Item = &ItemAbi> {
        self.abis.iter()
    }

    pub fn add_abi(&mut self, project: &mut Project, item_abi: &ItemAbi) {
        self.abis.push(item_abi.clone());
    }

    #[inline]
    pub fn traits(&self) -> impl Iterator<Item = &ItemTrait> {
        self.traits.iter()
    }

    pub fn add_trait(&mut self, project: &mut Project, item_trait: &ItemTrait) {
        self.traits.push(item_trait.clone());
    }

    #[inline]
    pub fn type_aliases(&self) -> impl Iterator<Item = &ItemTypeAlias> {
        self.types_aliases.iter()
    }

    pub fn add_type_alias(&mut self, project: &mut Project, item_type_alias: &ItemTypeAlias) {
        self.types_aliases.push(item_type_alias.clone());
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
                    ExprTupleDescriptor::Nil => empty_tuple_ty(),

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
                match block.inner.final_expr_opt.as_ref() {
                    Some(expr) => self.get_expr_ty(expr, project),
                    None => empty_tuple_ty()
                }
            }

            Expr::Array(array) => {
                match &array.inner {
                    ExprArrayDescriptor::Sequence(sequence) => {
                        if let Some((expr, _)) = sequence.value_separator_pairs.first() {
                            self.get_expr_ty(expr, project)
                        } else if let Some(expr) = sequence.final_value_opt.as_ref() {
                            self.get_expr_ty(expr, project)
                        } else {
                            empty_tuple_ty()
                        }
                    }

                    ExprArrayDescriptor::Repeat { value, .. } => {
                        self.get_expr_ty(value, project)
                    }
                }
            }

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
                                                final_value_opt: Some(Box::new(ty)),
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

            Expr::Reassignment { .. } => empty_tuple_ty(),

            Expr::Break { .. } | Expr::Continue { .. } => empty_tuple_ty(),
        }
    }

    fn expand_pattern(&self, project: &mut Project, pattern: &Pattern) -> Pattern {
        match pattern {
            Pattern::Or { lhs, pipe_token, rhs } => todo!(),
            Pattern::Wildcard { underscore_token } => todo!(),
            Pattern::AmbiguousSingleIdent(base_ident) => Pattern::AmbiguousSingleIdent(base_ident.clone()),
            Pattern::Var { reference, mutable, name } => todo!(),
            Pattern::Literal(_) => todo!(),
            Pattern::Constant(_) => todo!(),
            Pattern::Constructor { path, args } => todo!(),
            Pattern::Struct { path, fields } => todo!(),
            Pattern::Tuple(_) => todo!(),
            Pattern::Error(_, _) => todo!(),
        }
    }
    

    
    fn expand_path_expr(&self, project: &mut Project, path_expr: &PathExpr) -> PathExpr {
        //
        // TODO: resolve full path expr
        //

        match path_expr.root_opt.as_ref() {
            Some(_) => {
                //
                // TODO: find the module in the project
                //

                todo!()
            }

            None => {
                // Look for a library
                let resolver = project.resolver.clone();

                let mut result = None;
                
                for library in resolver.borrow().libraries.iter() {
                    if library.name == path_expr.prefix.name.as_str() {
                        result = Some(PathExpr {
                            root_opt: path_expr.root_opt.clone(),
                            prefix: path_expr.prefix.clone(),
                            suffix: vec![],
                            incomplete_suffix: path_expr.incomplete_suffix.clone(),
                        });
                        break;
                    }
                }

                let mut result = result.unwrap();

                result.suffix.extend(path_expr.suffix.clone());

                result
            }
        }
    }

    fn expand_path_type(&self, project: &mut Project, path_type: &PathType) -> PathType {
        //
        // TODO:
        // Turn relative path into full path, i.e: `StorageKey<Option<T>>` => `core::storage::StorageKey<std::option::Option<T>>`
        // We should check the `core::prelude` and `std::prelude` modules first before checking the `use` statements in scope.
        //
        match path_type.root_opt.as_ref() {
            Some(_) => {
                //
                // TODO: find the module in the project
                //

                todo!()
            }

            None => {
                // Look for a library
                let resolver = project.resolver.clone();

                // Check if we are looking for a standard type


                // 1. Look for a type in the current module
                match path_type.prefix.generics_opt.as_ref() {
                    None => {
                        // Check for a type alias in the current module
                        if let Some(ItemTypeAlias{  ty: Ty::Path(path_ty), .. }) = self.types_aliases.iter().find(|alias| {
                            alias.name.as_str() == path_type.prefix.name.as_str()
                        }) {
                            return self.expand_path_type(project, path_ty);
                        }

                        // Check for a abi in the current module 
                        if let Some(item_abi) = self.abis.iter().find(|alias| {
                            alias.name.as_str() == path_type.prefix.name.as_str()
                        }) {
                            return PathType { 
                                root_opt: None, 
                                prefix: PathTypeSegment { 
                                    name: item_abi.name.clone(), 
                                    generics_opt: None 
                                }, 
                                suffix: vec![] 
                            };
                        }
                    }

                    Some((_, generics)) => {
                        // Count the number of generic parameters
                        let mut input_count = 0;
                        for _ in &generics.parameters.inner {
                            input_count += 1;
                        }

                        // Check for a struct in the current module
                        if let Some(item_struct) = self.structs.iter().find(|x| {
                            x.name.as_str() == path_type.prefix.name.as_str() && x.generics.as_ref().map(|x| {
                                let mut count = 0;
                                for _ in &x.parameters.inner {
                                    count += 1;
                                }
                                count == input_count
                            }).unwrap_or(false) 
                        }) {
                            return PathType { 
                                root_opt: None, 
                                prefix: PathTypeSegment { 
                                    name: item_struct.name.clone(), 
                                    generics_opt: Some((None, generics.clone())) 
                                }, 
                                suffix: vec![] 
                            };
                        }  

                        // Check for a trait in the current module
                        if let Some(item_trait) = self.traits.iter().find(|x| {
                            x.name.as_str() == path_type.prefix.name.as_str() && x.generics.as_ref().map(|x| {
                                let mut count = 0;
                                for _ in &x.parameters.inner {
                                    count += 1;
                                }
                                count == input_count
                            }).unwrap_or(false) 
                        }) {
                            return PathType { 
                                root_opt: None, 
                                prefix: PathTypeSegment { 
                                    name: item_trait.name.clone(), 
                                    generics_opt: Some((None, generics.clone())) 
                                }, 
                                suffix: vec![] 
                            };
                        }  
                    }
                }
                
                // 2. Look an explicit `use` statement in the current module
                if self.uses.iter().find(|item_use| {
                    let flatten_use_tree = flatten_use_tree(None, &item_use.tree);

                    for path_expr in &flatten_use_tree {
                        if path_expr.prefix.name.as_str() == path_type.prefix.name.as_str() {
                            return true;
                        }
                    }
                    return false;
                }).is_some() {
                    return path_type.clone();
                }

                let mut check_library = |library_name: &str| -> Option<PathType> {
                    if let Some(library) = resolver.borrow().libraries.iter().find(|lib| lib.name == library_name) {
                        let Some(prelude) = library.modules.iter().find(|module| {
                            module.name.as_str() == "prelude"
                        }) else {
                            panic!("std::prelude not found");
                        };
                        
                        match path_type.prefix.generics_opt.as_ref() {                        
                            None => {
                                // Check for a type alias in the current module
                                for item in &prelude.inner.items {
                                    let ItemKind::TypeAlias(item_type_alias) = &item.value else {
                                        continue;
                                    };
                                    if item_type_alias.name.as_str() == path_type.prefix.name.as_str() {
                                        let mut expanded_path_type = self.expand_path_type(project, path_type);
                                        let prefix = expanded_path_type.prefix.clone();
                                        
                                        expanded_path_type.prefix = PathTypeSegment {
                                            name: BaseIdent::new_no_span(library_name.to_string()),
                                            generics_opt: None
                                        };
                                        
                                        expanded_path_type.suffix.insert(0, (DoubleColonToken::new(Span::dummy()), prefix));
                                        
                                        return Some(expanded_path_type);
                                    }
                                }
    
                                // Check for a abi in the current module
                                for item in &prelude.inner.items {
                                    let ItemKind::Abi(item_abi) = &item.value else {
                                        continue;
                                    };
                                    if item_abi.name.as_str() == path_type.prefix.name.as_str() {
                                        let mut expanded_path_type = self.expand_path_type(project, path_type);
                                        let prefix = expanded_path_type.prefix.clone();
                                        
                                        expanded_path_type.prefix = PathTypeSegment {
                                            name: BaseIdent::new_no_span(library_name.to_string()),
                                            generics_opt: None
                                        };
                                        
                                        expanded_path_type.suffix.insert(0, (DoubleColonToken::new(Span::dummy()), prefix));
                                        
                                        return Some(expanded_path_type);
                                    }
                                }
                            }
    
                            Some((_, generics)) => {
                                // Count the number of generic parameters
                                let mut input_count = 0;
                                for _ in &generics.parameters.inner {
                                    input_count += 1;
                                }
                                
                                // Check for a struct in the current module
                                for item in &prelude.inner.items {
                                    let ItemKind::Struct(item_struct) = &item.value else {
                                        continue;
                                    };
                                    if item_struct.name.as_str() == path_type.prefix.name.as_str() && item_struct.generics.as_ref().map(|x| {
                                        let mut count = 0;
                                        for _ in &x.parameters.inner {
                                            count += 1;
                                        }
                                        count == input_count
                                    }).unwrap_or(false) {
                                        let mut expanded_path_type = PathType { 
                                            root_opt: None, 
                                            prefix: PathTypeSegment { 
                                                name: item_struct.name.clone(), 
                                                generics_opt: Some((None, generics.clone())) 
                                            }, 
                                            suffix: vec![] 
                                        };
                                        
                                        let prefix = expanded_path_type.prefix.clone();
                                        
                                        expanded_path_type.prefix = PathTypeSegment {
                                            name: BaseIdent::new_no_span(library_name.to_string()),
                                            generics_opt: None
                                        };
                                        expanded_path_type.suffix.insert(0, (DoubleColonToken::new(Span::dummy()), prefix));
                                        
                                        return Some(expanded_path_type);
                                    }
                                }
    
                                // Check for a trait in the current module
                                for item in &prelude.inner.items {
                                    let ItemKind::Trait(item_trait) = &item.value else {
                                        continue;
                                    };
                                    if item_trait.name.as_str() == path_type.prefix.name.as_str() && item_trait.generics.as_ref().map(|x| {
                                        let mut count = 0;
                                        for _ in &x.parameters.inner {
                                            count += 1;
                                        }
                                        count == input_count
                                    }).unwrap_or(false) {
                                        let mut expanded_path_type = PathType { 
                                            root_opt: None, 
                                            prefix: PathTypeSegment { 
                                                name: item_trait.name.clone(), 
                                                generics_opt: Some((None, generics.clone())) 
                                            }, 
                                            suffix: vec![] 
                                        };
                                        
                                        let prefix = expanded_path_type.prefix.clone();
                                        expanded_path_type.prefix = PathTypeSegment {
                                            name: BaseIdent::new_no_span(library_name.to_string()),
                                            generics_opt: None
                                        };
                                        
                                        expanded_path_type.suffix.insert(0, (DoubleColonToken::new(Span::dummy()), prefix));
                                        
                                        return Some(expanded_path_type);
                                    }
                                }
                            }
                        }
                        
                        for item in &prelude.inner.items {
                            let ItemKind::Use(item_use) = &item.value else {
                                continue;
                            };

                            let flatten_use_tree = flatten_use_tree(None, &item_use.tree);
                            
                            for path_expr in &flatten_use_tree {
                                if path_expr.suffix.last().map(|(_,s)| s.name.as_str() == path_type.prefix.name.as_str()).unwrap_or(false) {
                                    let mut expanded_path_type = path_type.clone();
                    
                                    if item_use.root_import.is_some() {
                                        let prefix = expanded_path_type.prefix.clone();
                                        expanded_path_type.prefix = PathTypeSegment {
                                            name: BaseIdent::new_no_span(library_name.to_string()),
                                            generics_opt: None
                                        };

                                        expanded_path_type.suffix.insert(0, (DoubleColonToken::new(Span::dummy()), prefix));
                                    }
                                    
                                    return Some(path_type.clone());
                                }
                            }
                        }
                    }

                    None
                };

                // 3. Check the std prelude
                if let Some(result) = check_library("std") {
                    return result;
                }

                // 4. Check the core prelude
                if let Some(result) = check_library("core") {
                    return result;
                }

                let mut result = None;
                
                for library in resolver.borrow().libraries.iter() {
                    if library.name == path_type.prefix.name.as_str() {
                        result = Some(PathType {
                            root_opt: path_type.root_opt.clone(),
                            prefix: path_type.prefix.clone(),
                            suffix: vec![],
                        });
                        break;
                    }
                }

                if result.is_none() && path_type.prefix.generics_opt.is_none()  && path_type.suffix.is_empty() {
                    if let "u8" | "u16" | "u32" | "u64" | "u256" | "bool" | "str" | "b256" = path_type.prefix.name.as_str() {
                        return path_type.clone();
                    }
                }

                let mut result = result.unwrap();

                result.suffix.extend(path_type.suffix.clone());

                result
            }
        }
        // todo!("path_type : {:#?}", path_type)
    }

    fn expand_ty(&self, project: &mut Project, ty: &Ty) -> Ty {
        if project.resolver.borrow().resolve_ty(ty).is_some() {
            return ty.clone();
        }

        match ty {
            Ty::Path(path_type) => Ty::Path(self.expand_path_type(project, path_type)),
            
            Ty::Tuple(tuple) => Ty::Tuple(Parens {
                inner: match &tuple.inner {
                    TyTupleDescriptor::Nil => TyTupleDescriptor::Nil,
                    TyTupleDescriptor::Cons { head, comma_token, tail } => TyTupleDescriptor::Cons {
                        head: Box::new(self.expand_ty(project, head)),
                        comma_token: comma_token.clone(),
                        tail: Punctuated {
                            value_separator_pairs: tail.value_separator_pairs.iter()
                                .map(|(ty, comma)| (self.expand_ty(project, ty), comma.clone()))
                                .collect(),
                            final_value_opt: tail.final_value_opt.as_ref()
                                .map(|ty| Box::new(self.expand_ty(project, ty))),
                        },
                    },
                },
                span: tuple.span.clone(),
            }),

            Ty::Array(array) => Ty::Array(SquareBrackets {
                inner: TyArrayDescriptor {
                    ty: Box::new(self.expand_ty(project, &array.inner.ty)),
                    semicolon_token: array.inner.semicolon_token.clone(),
                    length: array.inner.length.clone(),
                },
                span: array.span.clone(),
            }),
            
            Ty::Ptr { ptr_token, ty } => Ty::Ptr {
                ptr_token: ptr_token.clone(),
                ty: SquareBrackets {
                    inner: Box::new(self.expand_ty(project, &ty.inner)),
                    span: ty.span.clone(),
                },
            },

            Ty::Slice { slice_token, ty } => Ty::Slice {
                slice_token: slice_token.clone(),
                ty: SquareBrackets {
                    inner: Box::new(self.expand_ty(project, &ty.inner)),
                    span: ty.span.clone(),
                },
            },

            Ty::Ref { ampersand_token, mut_token, ty } => Ty::Ref {
                ampersand_token: ampersand_token.clone(),
                mut_token: mut_token.clone(),
                ty: Box::new(self.expand_ty(project, ty)),
            },

            Ty::StringSlice(_) |
            Ty::StringArray { .. } |
            Ty::Infer { .. } |
            Ty::Never { .. } => ty.clone(),
        }
    }
}
