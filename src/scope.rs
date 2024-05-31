use crate::{project::Project, utils};
use std::{cell::RefCell, rc::Rc};
use sway_ast::{
    keywords::{CloseAngleBracketToken, Keyword, OpenAngleBracketToken, StrToken}, ty::TyTupleDescriptor, AngleBrackets, CommaToken, DoubleColonToken, Expr, ExprArrayDescriptor, ExprTupleDescriptor, FnArg, FnArgs, FnSignature, GenericArgs, GenericParams, ItemAbi, ItemEnum, ItemImpl, ItemImplItem, ItemKind, ItemStruct, ItemTrait, ItemTraitItem, ItemTypeAlias, ItemUse, Literal, MatchBranchKind, Parens, PathExpr, PathExprSegment, PathType, PathTypeSegment, Pattern, PatternStructField, Punctuated, Ty, UseTree, WhereClause
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
    fn_signatures: Vec<Rc<RefCell<FnSignature>>>,
    structs: Vec<Rc<RefCell<ItemStruct>>>,
    enums: Vec<Rc<RefCell<ItemEnum>>>,
    abis: Vec<Rc<RefCell<ItemAbi>>>,
    traits: Vec<Rc<RefCell<ItemTrait>>>,
    type_aliases: Vec<Rc<RefCell<ItemTypeAlias>>>,
    generic_params: Vec<Rc<RefCell<(GenericParams, Option<WhereClause>)>>>,
    impls: Vec<Rc<RefCell<ItemImpl>>>,
}

impl AstScope {
    /// Creates a new scope.
    pub fn new(parent: Option<Rc<RefCell<AstScope>>>) -> Self {
        Self {
            parent,
            ..Default::default()
        }
    }

    pub fn from_library(project: &mut Project, library_name: &str) -> Vec<(PathExpr, Rc<RefCell<AstScope>>)> {
        let mut result = vec![];

        let Some(library) = project.resolver.borrow().libraries.iter().find(|library| library.name == library_name).cloned() else {
            panic!("Failed to find library \"{library_name}\"");
        };

        let library_path = PathExpr {
            root_opt: None,
            prefix: PathExprSegment {
                name: BaseIdent::new_no_span(library_name.into()),
                generics_opt: None,
            },
            suffix: vec![],
            incomplete_suffix: false,
        };

        for module in library.modules.iter() {
            let mut module_path = library_path.clone();

            for module_path_part in module.name.split("::") {
                module_path.suffix.push(
                    (
                        DoubleColonToken::default(),
                        PathExprSegment {
                            name: BaseIdent::new_no_span(module_path_part.into()),
                            generics_opt: None,
                        }
                    )
                );
            }

            // println!("Module: {}", utils::path_expr_to_string(&module_path));

            let scope = Rc::new(RefCell::new(AstScope::default()));

            for module_item in module.inner.items.iter() {
                match &module_item.value {
                    ItemKind::Use(item_use) => {
                        let mut item_use = item_use.clone();

                        if item_use.root_import.is_some() {
                            match &mut item_use.tree {
                                UseTree::Path { prefix, suffix, .. } => {
                                    let old_prefix = prefix.clone();
                                    
                                    *prefix = BaseIdent::new_no_span(library_name.into());
                                    
                                    *suffix = Box::new(UseTree::Path {
                                        prefix: old_prefix,
                                        double_colon_token: DoubleColonToken::default(),
                                        suffix: suffix.clone(),
                                    });
                                }

                                tree => {
                                    *tree = UseTree::Path {
                                        prefix: BaseIdent::new_no_span(library_name.into()),
                                        double_colon_token: DoubleColonToken::default(),
                                        suffix: Box::new(tree.clone()),
                                    };
                                }
                            }

                            item_use.root_import = None;
                        }

                        scope.borrow_mut().add_use(&item_use);
                    }

                    ItemKind::Struct(item_struct) => {
                        scope.borrow_mut().add_struct(project, item_struct);
                    }

                    ItemKind::Enum(item_enum) => {
                        scope.borrow_mut().add_enum(project, item_enum);
                    }

                    ItemKind::Trait(item_trait) => {
                        scope.borrow_mut().add_trait(project, item_trait);
                    }

                    ItemKind::Abi(item_abi) => {
                        scope.borrow_mut().add_abi(project, item_abi);
                    }

                    ItemKind::TypeAlias(item_type_alias) => {
                        scope.borrow_mut().add_type_alias(project, item_type_alias);
                    }

                    _ => {}
                }
            }

            for module_item in module.inner.items.iter() {
                match &module_item.value {
                    ItemKind::Submodule(_) => {}
                    ItemKind::Use(_) => {}
                    ItemKind::Struct(_) => {}
                    ItemKind::Enum(_) => {}

                    ItemKind::Fn(item_fn) => {
                        scope.borrow_mut().add_fn_signature(project, &item_fn.fn_signature);
                    }

                    ItemKind::Trait(_) => {}

                    ItemKind::Impl(item_impl) => {
                        scope.borrow_mut().add_impl(project, item_impl);
                    }

                    ItemKind::Abi(item_abi) => {}

                    ItemKind::Const(item_const) => {
                        scope.borrow_mut().add_variable(
                            project,
                            AstVariableKind::Constant,
                            &item_const.name,
                            item_const.ty_opt.as_ref().map(|(_, ty)| ty).unwrap(),
                        );
                    }

                    ItemKind::Storage(item_storage) => {
                        for field in &item_storage.fields.inner {
                            scope.borrow_mut().add_variable(
                                project,
                                AstVariableKind::Storage,
                                &field.value.name,
                                &field.value.ty,
                            );
                        }
                    }

                    ItemKind::Configurable(item_configurable) => {
                        for field in &item_configurable.fields.inner {
                            scope.borrow_mut().add_variable(
                                project,
                                AstVariableKind::Configurable,
                                &field.value.name,
                                &field.value.ty,
                            );
                        }
                    }
                    
                    ItemKind::TypeAlias(_) => {}

                    ItemKind::Error(_, _) => panic!("Encountered an error while parsing Sway AST"),
                }
            }
        
            result.push((module_path, scope));
        }

        result
    }

    /// Gets the parent of the scope (if any).
    #[inline]
    pub fn parent(&self) -> Option<Rc<RefCell<AstScope>>> {
        self.parent.clone()
    }

    /// Gets an iterator over all of the `use` items in the current scope. This does not include parent scopes.
    #[inline]
    pub fn uses(&self) -> impl Iterator<Item = &Rc<RefCell<ItemUse>>> {
        self.uses.iter()
    }

    /// Attempts to find a `use` item in the current scope or any of its parents using the supplied lambda.
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
    
    /// Gets all glob `use` items in the current scope and all of its parents.
    pub fn get_glob_uses(&self) -> Vec<PathExpr> { 
        let mut out = vec![];

        if let Some(parent) = self.parent.as_ref() {
            out.extend(parent.borrow().get_glob_uses());
        }

        for item_use in self.uses.iter() {
            let path_exprs = utils::flatten_use_tree(None, &item_use.borrow().tree);

            for path_expr in path_exprs {
                let suffix = if let Some(suffix) = path_expr.suffix.last() {
                    &suffix.1
                } else {
                    &path_expr.prefix
                };

                if suffix.name.as_str() == "*" {
                    out.push(path_expr);
                }
            }
        }

        out
    }

    /// Adds a `use` item to the scope.
    #[inline]
    pub fn add_use(&mut self, item_use: &ItemUse) {
        //
        // TODO: ensure the use is not already declared
        //

        self.uses.push(Rc::new(RefCell::new(item_use.clone())));
    }

    /// Gets an iterator over all of the variables in the current scope. This does not include parent scopes.
    #[inline]
    pub fn variables(&self) -> impl Iterator<Item = &Rc<RefCell<AstVariable>>> {
        self.variables.iter()
    }

    /// Adds a variable to the scope.
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
            ty: self.expand_ty(project, ty, &[]),
        })));
    }

    /// Attempts to find a variable in the current scope or any of its parents using the supplied lambda.
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

    /// Gets an iterator over the all of the function signatures in the current scope. This does not include parent scopes.
    #[inline]
    pub fn fn_signatures(&self) -> impl Iterator<Item = &Rc<RefCell<FnSignature>>> {
        self.fn_signatures.iter()
    }

    /// Adds a function signature to the scope.
    #[inline]
    pub fn add_fn_signature(&mut self, project: &mut Project, fn_signature: &FnSignature) {
        self.fn_signatures.push(Rc::new(RefCell::new(self.expand_fn_signature(project, fn_signature, &[]))));
    }

    /// Gets an iterator over all of the `struct` items in the current scope. This does not include parent scopes.
    #[inline]
    pub fn structs(&self) -> impl Iterator<Item = &Rc<RefCell<ItemStruct>>> {
        self.structs.iter()
    }

    /// Attempts to find a `struct` item in the current scope or any of its parents using the supplied lambda.
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

    /// Adds a `struct` item to the scope.
    #[inline]
    pub fn add_struct(&mut self, project: &mut Project, item_struct: &ItemStruct) {
        let mut item_struct = item_struct.clone();

        let generic_idents = match item_struct.generics.as_ref() {
            Some(generics) => {
                let mut generic_idents = vec![];
                for generic in &generics.parameters.inner {
                    generic_idents.push(generic.clone());
                }
                generic_idents
            }

            None => vec![],
        };
        
        if let Some(where_clause) = item_struct.where_clause_opt.as_mut() {
            *where_clause = self.expand_where_clause(project, where_clause, &generic_idents);
        }

        for (field, _) in item_struct.fields.inner.value_separator_pairs.iter_mut() {
            field.value.ty = self.expand_ty(project, &field.value.ty, &generic_idents);
        }

        if let Some(field) = item_struct.fields.inner.final_value_opt.as_mut() {
            field.value.ty = self.expand_ty(project, &field.value.ty, &generic_idents);
        }

        self.structs.push(Rc::new(RefCell::new(item_struct)));
    }

    /// Gets an iterator over all of the `enum` items in the current scope. This does not include parent scopes.
    #[inline]
    pub fn enums(&self) -> impl Iterator<Item = &Rc<RefCell<ItemEnum>>> {
        self.enums.iter()
    }

    /// Attempts to find a `enum` item in the current scope or any of its parents using the supplied lambda.
    #[inline]
    pub fn find_enum<F: Copy + FnMut(&&Rc<RefCell<ItemEnum>>) -> bool>(&self, f: F) -> Option<Rc<RefCell<ItemEnum>>> {
        if let Some(x) = self.enums.iter().find(f) {
            return Some(x.clone());
        }

        if let Some(parent) = self.parent.as_ref() {
            if let Some(x) = parent.borrow().find_enum(f) {
                return Some(x.clone());
            }
        }

        None
    }

    /// Adds a `enum` item to the scope.
    #[inline]
    pub fn add_enum(&mut self, project: &mut Project, item_enum: &ItemEnum) {
        let mut item_enum = item_enum.clone();

        let generic_idents = match item_enum.generics.as_ref() {
            Some(generics) => {
                let mut generic_idents = vec![];
                for generic in &generics.parameters.inner {
                    generic_idents.push(generic.clone());
                }
                generic_idents
            }

            None => vec![],
        };
        
        if let Some(where_clause) = item_enum.where_clause_opt.as_mut() {
            *where_clause = self.expand_where_clause(project, where_clause, &generic_idents);
        }

        for (field, _) in item_enum.fields.inner.value_separator_pairs.iter_mut() {
            field.value.ty = self.expand_ty(project, &field.value.ty, &generic_idents);
        }

        if let Some(field) = item_enum.fields.inner.final_value_opt.as_mut() {
            field.value.ty = self.expand_ty(project, &field.value.ty, &generic_idents);
        }

        self.enums.push(Rc::new(RefCell::new(item_enum)));
    }

    #[inline]
    pub fn abis(&self) -> impl Iterator<Item = &Rc<RefCell<ItemAbi>>> {
        self.abis.iter()
    }

    /// Attempts to find an `abi` item in the current scope or any of its parents using the supplied lambda.
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

    /// Adds an `abi` item to the scope.
    #[inline]
    pub fn add_abi(&mut self, project: &mut Project, item_abi: &ItemAbi) {
        let mut item_abi = item_abi.clone();

        if let Some(super_traits) = item_abi.super_traits.as_mut() {
            super_traits.1.prefix = self.expand_path_type(project, &super_traits.1.prefix, &[]);
            
            for (_, suffix) in super_traits.1.suffixes.iter_mut() {
                *suffix = self.expand_path_type(project, suffix, &[]);
            }
        }

        for item in item_abi.abi_items.inner.iter_mut() {
            match &mut item.value {
                ItemTraitItem::Fn(fn_signature, _) => {
                    *fn_signature = self.expand_fn_signature(project, fn_signature, &[]);
                }

                ItemTraitItem::Const(item_const, _) => {
                    if let Some((_, ty)) = item_const.ty_opt.as_mut() {
                        *ty = self.expand_ty(project, &ty, &[]);
                    }
                }

                ItemTraitItem::Type(item_type, _) => {
                    if let Some(ty) = item_type.ty_opt.as_mut() {
                        *ty = self.expand_ty(project, &ty, &[]);
                    }
                }

                ItemTraitItem::Error(_, _) => panic!("Encountered an error while parsing Sway AST"),
            }
        }

        if let Some(abi_defs) = item_abi.abi_defs_opt.as_mut() {
            for item_fn in abi_defs.inner.iter_mut() {
                item_fn.value.fn_signature = self.expand_fn_signature(project, &item_fn.value.fn_signature, &[]);
                item_fn.value.body.inner.statements.clear();
                item_fn.value.body.inner.final_expr_opt = None;
            }
        }

        self.abis.push(Rc::new(RefCell::new(item_abi)));
    }

    /// Gets an iterator over all of the `abi` items in the current scope. This does not include parent scopes.
    #[inline]
    pub fn traits(&self) -> impl Iterator<Item = &Rc<RefCell<ItemTrait>>> {
        self.traits.iter()
    }

    /// Attempts to find a `trait` item in the current scope or any of its parents using the supplied lambda.
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

    /// Adds a `trait` item to the scope.
    #[inline]
    pub fn add_trait(&mut self, project: &mut Project, item_trait: &ItemTrait) {
        let mut item_trait = item_trait.clone();

        let generic_idents = match item_trait.generics.as_ref() {
            Some(generics) => {
                let mut generic_idents = vec![];
                for generic in &generics.parameters.inner {
                    generic_idents.push(generic.clone());
                }
                generic_idents
            }

            None => vec![],
        };
        
        if let Some(where_clause) = item_trait.where_clause_opt.as_mut() {
            *where_clause = self.expand_where_clause(project, where_clause, &generic_idents);
        }

        if let Some(super_traits) = item_trait.super_traits.as_mut() {
            super_traits.1.prefix = self.expand_path_type(project, &super_traits.1.prefix, &generic_idents);
            
            for (_, suffix) in super_traits.1.suffixes.iter_mut() {
                *suffix = self.expand_path_type(project, suffix, &generic_idents);
            }
        }

        for item in item_trait.trait_items.inner.iter_mut() {
            match &mut item.value {
                ItemTraitItem::Fn(fn_signature, _) => {
                    *fn_signature = self.expand_fn_signature(project, fn_signature, &generic_idents);
                }

                ItemTraitItem::Const(item_const, _) => {
                    if let Some((_, ty)) = item_const.ty_opt.as_mut() {
                        *ty = self.expand_ty(project, ty, &generic_idents);
                    }
                }

                ItemTraitItem::Type(item_type, _) => {
                    if let Some(ty) = item_type.ty_opt.as_mut() {
                        *ty = self.expand_ty(project, ty, &generic_idents);
                    }
                }

                ItemTraitItem::Error(_, _) => panic!("Encountered an error while parsing Sway AST"),
            }
        }

        if let Some(trait_defs) = item_trait.trait_defs_opt.as_mut() {
            for item_fn in trait_defs.inner.iter_mut() {
                item_fn.value.fn_signature = self.expand_fn_signature(project, &item_fn.value.fn_signature, &generic_idents);
                item_fn.value.body.inner.statements.clear();
                item_fn.value.body.inner.final_expr_opt = None;
            }
        }

        self.traits.push(Rc::new(RefCell::new(item_trait)));
    }

    /// Gets an iterator over all of the `type` items in the current scope. This does not include parent scopes.
    #[inline]
    pub fn type_aliases(&self) -> impl Iterator<Item = &Rc<RefCell<ItemTypeAlias>>> {
        self.type_aliases.iter()
    }

    /// Attempts to find a `type` item in the current scope or any of its parents using the supplied lambda.
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

    /// Adds a `type` item to the scope.
    #[inline]
    pub fn add_type_alias(&mut self, project: &mut Project, item_type_alias: &ItemTypeAlias) {
        let mut item_type_alias = item_type_alias.clone();
        
        item_type_alias.ty = self.expand_ty(project, &item_type_alias.ty, &[]);

        self.type_aliases.push(Rc::new(RefCell::new(item_type_alias)));
    }

    /// Gets an iterator over all of the generic arguments defined in the current scope. This does not include parent scopes.
    #[inline]
    pub fn generic_params(&self) -> impl Iterator<Item = &Rc<RefCell<(GenericParams, Option<WhereClause>)>>> {
        self.generic_params.iter()
    }

    /// Attempts to find a generic arguments defined in the current scope or any of its parents using the supplied lambda.
    #[inline]
    pub fn find_generic_params<F: Copy + FnMut(&&Rc<RefCell<(GenericParams, Option<WhereClause>)>>) -> bool>(&self, f: F) -> Option<Rc<RefCell<(GenericParams, Option<WhereClause>)>>> {
        if let Some(x) = self.generic_params.iter().find(f) {
            return Some(x.clone());
        }

        if let Some(parent) = self.parent.as_ref() {
            if let Some(x) = parent.borrow().find_generic_params(f) {
                return Some(x.clone());
            }
        }

        None
    }

    /// Adds generic arguments to the scope.
    #[inline]
    pub fn add_generic_params(&mut self, project: &mut Project, generic_params: &GenericParams, where_clause: Option<&WhereClause>) {
        let mut generic_idents = vec![];
        
        for generic in &generic_params.parameters.inner {
            generic_idents.push(generic.clone());
        }
        
        let mut where_clause = where_clause.cloned();
        
        if let Some(where_clause) = where_clause.as_mut() {
            *where_clause = self.expand_where_clause(project, where_clause, &generic_idents);
        }

        self.generic_params.push(Rc::new(RefCell::new((generic_params.clone(), where_clause))));
    }

    /// Gets an iterator over all of the `abi` items in the current scope. This does not include parent scopes.
    #[inline]
    pub fn impls(&self) -> impl Iterator<Item = &Rc<RefCell<ItemImpl>>> {
        self.impls.iter()
    }

    /// Attempts to find a `impl` item in the current scope or any of its parents using the supplied lambda.
    #[inline]
    pub fn find_impl<F: Copy + FnMut(&&Rc<RefCell<ItemImpl>>) -> bool>(&self, f: F) -> Option<Rc<RefCell<ItemImpl>>> {
        if let Some(x) = self.impls.iter().find(f) {
            return Some(x.clone());
        }

        if let Some(parent) = self.parent.as_ref() {
            if let Some(x) = parent.borrow().find_impl(f) {
                return Some(x.clone());
            }
        }

        None
    }

    /// Adds a `impl` item to the scope.
    #[inline]
    pub fn add_impl(&mut self, project: &mut Project, item_impl: &ItemImpl) {
        let mut item_impl = item_impl.clone();

        let generic_idents = match item_impl.generic_params_opt.as_ref() {
            Some(generics) => {
                let mut generic_idents = vec![];
                for generic in &generics.parameters.inner {
                    generic_idents.push(generic.clone());
                }
                generic_idents
            }

            None => vec![],
        };
        
        if let Some((path_type, _)) = item_impl.trait_opt.as_mut() {
            *path_type = self.expand_path_type(project, path_type, &generic_idents);
        }

        item_impl.ty = self.expand_ty(project, &item_impl.ty, &generic_idents);
        
        if let Some(where_clause) = item_impl.where_clause_opt.as_mut() {
            *where_clause = self.expand_where_clause(project, where_clause, &generic_idents);
        }

        for item in item_impl.contents.inner.iter_mut() {
            match &mut item.value {
                ItemImplItem::Fn(item_fn) => {
                    item_fn.fn_signature = self.expand_fn_signature(project, &item_fn.fn_signature, &generic_idents);
                    item_fn.body.inner.statements.clear();
                    item_fn.body.inner.final_expr_opt = None;
                }

                ItemImplItem::Const(item_const) => {
                    if let Some((_, ty)) = item_const.ty_opt.as_mut() {
                        *ty = self.expand_ty(project, ty, &generic_idents);
                    }
                }

                ItemImplItem::Type(item_type) => {
                    if let Some(ty) = item_type.ty_opt.as_mut() {
                        *ty = self.expand_ty(project, ty, &generic_idents);
                    }
                }
            }
        }

        self.impls.push(Rc::new(RefCell::new(item_impl)));
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
                Literal::Char(_) => utils::create_ident_ty("char"),
                Literal::Int(_) => utils::create_ident_ty("u64"),
                Literal::Bool(_) => utils::create_ident_ty("bool"),
            }

            Expr::AbiCast { args, .. } => Ty::Path(args.inner.name.clone()),

            Expr::Struct { path, fields: _ } => {
                //
                // TODO: check fields to make sure we are resolving the correct struct
                //

                let path_expr = self.expand_path_expr(project, path, &[]);
                let path_type = utils::path_expr_to_path_type(&path_expr);
                Ty::Path(path_type)
            }

            Expr::Tuple(tuple) => match &tuple.inner {
                ExprTupleDescriptor::Nil => utils::empty_tuple_ty(),

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
            }

            Expr::Parens(parens) => self.get_expr_ty(parens.inner.as_ref(), project),

            Expr::Block(block) => match block.inner.final_expr_opt.as_ref() {
                Some(expr) => self.get_expr_ty(expr, project),
                None => utils::empty_tuple_ty(),
            }

            Expr::Array(array) => match &array.inner {
                ExprArrayDescriptor::Sequence(sequence) => {
                    if let Some((expr, _)) = sequence.value_separator_pairs.first() {
                        self.get_expr_ty(expr, project)
                    } else if let Some(expr) = sequence.final_value_opt.as_ref() {
                        self.get_expr_ty(expr, project)
                    } else {
                        utils::empty_tuple_ty()
                    }
                }

                ExprArrayDescriptor::Repeat { value, .. } => self.get_expr_ty(value, project),
            }

            Expr::Asm(_) => {
                //
                // TODO: Get the type of the return value from the asm block if any
                //

                utils::empty_tuple_ty()
            }

            Expr::Return { .. } => utils::empty_tuple_ty(),

            Expr::If(if_expr) => {
                if let Some(expr) = if_expr.then_block.inner.final_expr_opt.as_ref() {
                    return self.get_expr_ty(expr, project);
                }

                utils::empty_tuple_ty()
            }

            Expr::Match { branches, .. } => {
                if let Some(branch) = branches.inner.first() {
                    match &branch.kind {
                        MatchBranchKind::Block { block, .. } => {
                            if let Some(expr) = block.inner.final_expr_opt.as_ref() {
                                return self.get_expr_ty(expr, project);
                            }

                            return utils::empty_tuple_ty();
                        }

                        MatchBranchKind::Expr { expr, .. } => {
                            return self.get_expr_ty(expr, project);
                        }
                    }
                }

                utils::empty_tuple_ty()
            }

            Expr::While { .. } | Expr::For { .. } => utils::empty_tuple_ty(),

            Expr::FuncApp { func: _, args: _ } => todo!("{expr:#?}"),

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
                    .unwrap_or_else(utils::empty_tuple_ty);

                self.expand_ty(project, &ty, &[])
            }

            Expr::FieldProjection { target, name, .. } => {
                // Check if the field projection refers to a storage field and return a `core::storage::StorageKey<T>` type
                if let Expr::Path(PathExpr { root_opt, prefix, suffix, .. }) = target.as_ref() {
                    if root_opt.is_none() && prefix.name.as_str() == "storage" && suffix.is_empty() {
                        let variable = self.get_variable(name.as_str(), true).unwrap();
                        let ty = self.expand_ty(project, &variable.borrow().ty, &[]);

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

                todo!()
                // let target_type = self.get_expr_ty(target, project);

                // let resolver = project.resolver.borrow();
                // let resolved = resolver.resolve_ty(&target_type);

                // let Some(sway_ast::ItemKind::Struct(item_struct)) = resolved else {
                //     panic!("Expected struct, found: {resolved:#?}")
                // };

                // let mut fields = vec![];

                // for field in &item_struct.fields.inner {
                //     fields.push(field);
                // }

                // let Some(field) = fields.iter().find(|f| f.value.name == *name) else {
                //     todo!("{expr:#?}")
                // };

                // field.value.ty.clone()
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

            Expr::Reassignment { .. } => utils::empty_tuple_ty(),

            Expr::Break { .. } | Expr::Continue { .. } => utils::empty_tuple_ty(),
        }
    }

    pub fn get_fn_signature(
        &self,
        _project: &mut Project,
        _fn_name: &PathExprSegment,
        _args: &Parens<Punctuated<Expr, CommaToken>>,
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
    ) -> Option<FnSignature> {
        //
        // TODO:
        //
        // We need to find a valid `impl` that contains the `fn` we are looking for.
        // We need to ensure the argument types of the `fn` match the types of the supplied `args`.
        //
        // If the `impl` is not defined in the current module, we need to find a `use` statement that imports a valid `impl` containing the `fn`:
        // 1. Check all explicit `use` statements
        // 2. Check `prelude` module of the `core` library
        // 3. Check `prelude` module of the `std` library
        // 
        //
        // Once we find the `impl` containing the `fn`, return the signature of the `fn`
        //

        let mut arg_types = vec![];

        for arg in &args.inner {
            arg_types.push(self.get_expr_ty(arg, project));
        }

        let mut check_scope = |scope: &AstScope| -> Option<FnSignature> {
            for mut path_expr in scope.get_glob_uses() {
                if let Some(suffix) = path_expr.suffix.last() {
                    if suffix.1.name.as_str() != "*" {
                        continue;
                    }

                    path_expr.suffix.pop();
                }

                let resolver = project.resolver.clone();
                let resolver = resolver.borrow();

                let Some(module) = resolver.resolve_module(&path_expr) else { continue };

                for item in module.inner.items.iter() {
                    let ItemKind::Impl(item_impl) = &item.value else { continue };
                    
                    let generic_idents = match item_impl.generic_params_opt.as_ref() {
                        Some(generics) => {
                            let mut generic_idents = vec![];
                            for generic in &generics.parameters.inner {
                                generic_idents.push(generic.clone());
                            }
                            generic_idents
                        }

                        None => vec![],
                    };
                    
                    let impl_ty = self.expand_ty(project, &item_impl.ty, &generic_idents);

                    if !self.is_ty_equivalent(&impl_ty, &ty) {
                        continue;
                    }
                    
                    for i in &item_impl.contents.inner {
                        let ItemImplItem::Fn(function) = &i.value else { continue };

                        if function.fn_signature.name.span().as_str() != fn_name.name.span().as_str() {
                            continue;
                        }
                        
                        let args = match &function.fn_signature.arguments.inner {
                            FnArgs::Static(args) => Some(args),
                            FnArgs::NonStatic { args_opt, .. } => args_opt.as_ref().map(|(_, args)| args),
                        };

                        let Some(args) = args else { continue };

                        let mut fn_arg_types = vec![];
                        
                        for arg in args {
                            fn_arg_types.push(self.expand_ty(project, &arg.ty, &generic_idents));
                        }
                        
                        if fn_arg_types.len() != arg_types.len() {
                            continue;
                        }

                        if fn_arg_types.iter().zip(arg_types.iter()).all(|(a, b)| self.is_ty_equivalent(a, b)) {
                            return Some(self.expand_fn_signature(project, &function.fn_signature, &generic_idents));
                        }
                    }
                }
            }

            None
        };

        if let Some(fn_signature) = check_scope(self) {
            return Some(fn_signature);
        }

        let mut parent = self.parent.clone();

        while let Some(scope) = parent {
            if let Some(fn_signature) = check_scope(&scope.borrow()) {
                return Some(fn_signature);
            }

            parent = scope.borrow().parent.clone();
        }
        
        todo!("Get function signature for {}::{}{}", utils::ty_to_string(ty), fn_name.name.span().as_str(), args.span.as_str())
    }

    #[inline]
    fn expand_path_type(&self, project: &mut Project, path_type: &PathType, generic_idents: &[BaseIdent]) -> PathType {
        utils::path_expr_to_path_type(
            &self.expand_path_expr(
                project,
                &utils::path_type_to_path_expr(path_type),
                generic_idents,
            ),
        )
    }

    fn expand_path_expr(&self, project: &mut Project, path_expr: &PathExpr, generic_idents: &[BaseIdent]) -> PathExpr {
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
                *ty = self.expand_ty(project, ty, generic_idents);
            }

            if let Some(ty) = generics.parameters.inner.final_value_opt.as_mut() {
                *ty = Box::new(self.expand_ty(project, ty.as_ref(), generic_idents));
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
                                    &self.expand_path_type(project, path_type, generic_idents),
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

                // 4. Check for an enum in the current module
                if let Some(item_enum) = self.find_enum(|item_enum| {
                    let item_enum = item_enum.borrow();
                    
                    if item_enum.name.as_str() != segment.name.as_str() {
                        return false;
                    }
                    
                    item_enum.generics.as_ref().map(|x| {
                        let mut count = 0;
                        for _ in &x.parameters.inner {
                            count += 1;
                        }
                        count
                    }).unwrap_or(0) == input_generic_count
                }) {
                    let item_enum = item_enum.borrow();

                    //
                    // TODO: find absolute path of current module and include it below
                    //

                    return PathExpr {
                        root_opt: None,
                        prefix: PathExprSegment {
                            name: item_enum.name.clone(),
                            generics_opt: segment.generics_opt.clone(),
                        },
                        suffix: vec![],
                        incomplete_suffix: false,
                    };
                }

                // 5. Check for a trait in the current module
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
                
                // 6. Look an explicit `use` statement in the current module
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
                                let mut expanded_path = self.expand_path_expr(project, path_expr, generic_idents);
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
                            let mut expanded_path = self.expand_path_expr(project, path_expr, generic_idents);
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

                // 7. Check the std prelude
                if let Some(result) = check_library_prelude("std") {
                    return result;
                }

                // 8. Check the core prelude
                if let Some(result) = check_library_prelude("core") {
                    return result;
                }

                // 9. Check any available libraries
                for library in resolver.borrow().libraries.iter() {
                    if library.name == segment.name.as_str() {
                        return path_expr.clone();
                    }
                }

                if path_expr.prefix.generics_opt.is_none() && path_expr.suffix.is_empty() {
                    // 10. Check for generic parameter types
                    if self.find_generic_params(|generic_params| {
                        for generic_param in &generic_params.borrow().0.parameters.inner {
                            if generic_param.as_str() == segment.name.as_str() {
                                return true;
                            }
                        }
                        false
                    }).is_some() {
                        return path_expr.clone();
                    }

                    if generic_idents.iter().find(|x| x.as_str() == segment.name.as_str()).is_some() {
                        return path_expr.clone();
                    }

                    // 11. Check for built-in types
                    if let "u8" | "u16" | "u32" | "u64" | "u256" | "bool" | "str" | "b256" | "raw_ptr" | "raw_slice" | "Self" | "Contract" = segment.name.as_str() {
                        return path_expr.clone();
                    }
                }

                // 12. Look through glob imports in the current module
                // TODO: Move this higher up
                for mut module_path in self.get_glob_uses() {
                    if let Some(suffix) = module_path.suffix.last() {
                        if suffix.1.name.as_str() != "*" {
                            continue;
                        }

                        module_path.suffix.pop();
                    }

                    let resolver = project.resolver.clone();
                    let resolver = resolver.borrow();

                    let Some(module) = resolver.resolve_module(&module_path) else { continue };

                    for item in module.inner.items.iter() {
                        //
                        // TODO: possibly extend this for function and variable lookups
                        //

                        let (name, generics) = match &item.value {
                            ItemKind::Struct(item) => (item.name.as_str(), item.generics.as_ref()),
                            ItemKind::Enum(item) => (item.name.as_str(), item.generics.as_ref()),
                            ItemKind::Trait(item) => (item.name.as_str(), item.generics.as_ref()),
                            ItemKind::Abi(item) => (item.name.as_str(), None),
                            ItemKind::TypeAlias(item) => (item.name.as_str(), None),
                            _ => continue,
                        };

                        if name == segment.name.as_str() {
                            if let Some(generics) = generics {
                                let mut generic_count = 0;

                                for _ in &generics.parameters.inner {
                                    generic_count += 1;
                                }

                                if generic_count != input_generic_count {
                                    continue;
                                }
                            }

                            let mut expanded_path = module_path.clone();
                            expanded_path.suffix.push((DoubleColonToken::default(), segment.clone()));
                            return expanded_path;
                        }
                    }
                }

                todo!("Failed to resolve path expression: {:#?}", path_expr)
            }
        }
    }

    fn expand_ty(&self, project: &mut Project, ty: &Ty, generic_idents: &[BaseIdent]) -> Ty {
        let mut ty = ty.clone();

        match &mut ty {
            Ty::Path(path_type) => {
                *path_type = self.expand_path_type(project, path_type, generic_idents);
            }

            Ty::Tuple(tuple) => {
                if let TyTupleDescriptor::Cons { head, tail, .. } = &mut tuple.inner {
                    *head.as_mut() = self.expand_ty(project, head.as_ref(), generic_idents);
                    
                    for (ty, _) in tail.value_separator_pairs.iter_mut() {
                        *ty = self.expand_ty(project, ty, generic_idents);
                    }

                    if let Some(ty) = tail.final_value_opt.as_mut() {
                        *ty.as_mut() = self.expand_ty(project, ty, generic_idents);
                    }
                }
            }

            Ty::Array(array) => {
                *array.inner.ty.as_mut() = self.expand_ty(project, array.inner.ty.as_ref(), generic_idents);
            }

            Ty::Ptr { ty, .. } | Ty::Slice { ty, .. } => {
                *ty.inner.as_mut() = self.expand_ty(project, ty.inner.as_ref(), generic_idents);
            }

            Ty::Ref { ty, .. } => {
                *ty.as_mut() = self.expand_ty(project, ty.as_ref(), generic_idents);
            }

            _ => {}
        }

        ty
    }

    #[inline]
    fn expand_fn_arg(&self, project: &mut Project, fn_arg: &FnArg, generic_idents: &[BaseIdent]) -> FnArg {
        let mut fn_arg = fn_arg.clone();
        fn_arg.pattern = self.expand_pattern(project, &fn_arg.pattern, generic_idents);
        fn_arg.ty = self.expand_ty(project, &fn_arg.ty, generic_idents);
        fn_arg
    }

    #[inline]
    fn expand_fn_signature(&self, project: &mut Project, fn_signature: &FnSignature, generic_idents: &[BaseIdent]) -> FnSignature {
        let mut fn_signature = fn_signature.clone();

        let mut generic_idents = generic_idents.iter().cloned().collect::<Vec<_>>();
        
        if let Some(generics) = fn_signature.generics.as_ref() {
            for generic in &generics.parameters.inner {
                generic_idents.push(generic.clone());
            }
        }

        if let Some(args) = match &mut fn_signature.arguments.inner {
            FnArgs::Static(args) => Some(args),
            FnArgs::NonStatic { args_opt, .. } => args_opt.as_mut().map(|(_, args)| args),
        } {
            for (arg, _) in args.value_separator_pairs.iter_mut() {
                self.expand_fn_arg(project, arg, &generic_idents);
            }

            if let Some(arg) = args.final_value_opt.as_mut() {
                self.expand_fn_arg(project, arg, &generic_idents);
            }
        }

        if let Some((_, return_type)) = fn_signature.return_type_opt.as_mut() {
            *return_type = self.expand_ty(project, return_type, &generic_idents);
        }

        if let Some(where_clause) = fn_signature.where_clause_opt.as_mut() {
            *where_clause = self.expand_where_clause(project, where_clause, &generic_idents);
        }

        fn_signature
    }

    #[inline]
    fn expand_where_clause(&self, project: &mut Project, where_clause: &WhereClause, generic_idents: &[BaseIdent]) -> WhereClause {
        let mut where_clause = where_clause.clone();

        for (where_bound, _) in where_clause.bounds.value_separator_pairs.iter_mut() {
            where_bound.bounds.prefix = self.expand_path_type(project, &where_bound.bounds.prefix, generic_idents);

            for (_, suffix) in where_bound.bounds.suffixes.iter_mut() {
                *suffix = self.expand_path_type(project, suffix, generic_idents);
            }
        }

        if let Some(where_bound) = where_clause.bounds.final_value_opt.as_mut() {
            where_bound.bounds.prefix = self.expand_path_type(project, &where_bound.bounds.prefix, generic_idents);

            for (_, suffix) in where_bound.bounds.suffixes.iter_mut() {
                *suffix = self.expand_path_type(project, suffix, generic_idents);
            }
        }
        
        where_clause
    }

    fn expand_pattern(&self, project: &mut Project, pattern: &Pattern, generic_idents: &[BaseIdent]) -> Pattern {
        let mut pattern = pattern.clone();

        match &mut pattern {
            Pattern::Or { lhs, rhs, .. } => {
                *lhs.as_mut() = self.expand_pattern(project, lhs, generic_idents);
                *rhs.as_mut() = self.expand_pattern(project, rhs, generic_idents);
            }

            Pattern::Constant(path_expr) => {
                *path_expr = self.expand_path_expr(project, path_expr, generic_idents);
            }
            
            Pattern::Constructor { path, args } => {
                *path = self.expand_path_expr(project, path, generic_idents);

                for (arg, _) in args.inner.value_separator_pairs.iter_mut() {
                    *arg = self.expand_pattern(project, arg, generic_idents);
                }
                
                if let Some(arg) = args.inner.final_value_opt.as_mut() {
                    *arg.as_mut() = self.expand_pattern(project, arg, generic_idents);
                }
            }

            Pattern::Struct { path, fields } => {
                *path = self.expand_path_expr(project, path, generic_idents);
                
                for (field, _) in fields.inner.value_separator_pairs.iter_mut() {
                    if let PatternStructField::Field { pattern_opt: Some((_, pattern)), .. } = field {
                        *pattern.as_mut() = self.expand_pattern(project, pattern, generic_idents);
                    }
                }

                if let Some(field) = fields.inner.final_value_opt.as_mut() {
                    if let PatternStructField::Field { pattern_opt: Some((_, pattern)), .. } = field.as_mut() {
                        *pattern.as_mut() = self.expand_pattern(project, pattern, generic_idents);
                    }
                }
            }
            
            Pattern::Tuple(tuple) => {
                for (pattern, _) in tuple.inner.value_separator_pairs.iter_mut() {
                    *pattern = self.expand_pattern(project, pattern, generic_idents);
                }

                if let Some(pattern) = tuple.inner.final_value_opt.as_mut() {
                    *pattern.as_mut() = self.expand_pattern(project, pattern, generic_idents);
                }
            }

            _ => {}
        }
    
        pattern
    }

    fn is_ty_equivalent(&self, lhs: &Ty, rhs: &Ty) -> bool {
        match (lhs, rhs) {
            (Ty::Path(lhs), Ty::Path(rhs)) => {
                if lhs.suffix.len() != rhs.suffix.len() {
                    return false;
                }

                let is_path_type_segment_equivalent = |lhs: &PathTypeSegment, rhs: &PathTypeSegment| -> bool {
                    if lhs.name.as_str() != rhs.name.as_str() {
                        return false;
                    }

                    if lhs.generics_opt.is_some() != rhs.generics_opt.is_some() {
                        return false;
                    }
    
                    match (lhs.generics_opt.as_ref(), rhs.generics_opt.as_ref()) {
                        (Some(lhs), Some(rhs)) => {
                            if lhs.1.parameters.inner.value_separator_pairs.len() != rhs.1.parameters.inner.value_separator_pairs.len() {
                                return false;
                            }
    
                            if lhs.1.parameters.inner.final_value_opt.is_some() != rhs.1.parameters.inner.final_value_opt.is_some() {
                                return false;
                            }
    
                            for ((lhs, _), (rhs, _)) in lhs.1.parameters.inner.value_separator_pairs.iter().zip(rhs.1.parameters.inner.value_separator_pairs.iter()) {
                                if !self.is_ty_equivalent(&lhs, &rhs) {
                                    return false;
                                }
                            }
    
                            if let (Some(lhs), Some(rhs)) = (&lhs.1.parameters.inner.final_value_opt, &rhs.1.parameters.inner.final_value_opt) {
                                if !self.is_ty_equivalent(lhs, rhs) {
                                    return false;
                                }
                            }
                            true
                        }

                        (None, None) => true,

                        _ => false,
                    }
                };

                if !is_path_type_segment_equivalent(&lhs.prefix, &rhs.prefix) {
                    return false;
                }

                for ((_, lhs), (_, rhs)) in lhs.suffix.iter().zip(rhs.suffix.iter()) {
                    if !is_path_type_segment_equivalent(lhs, rhs) {
                        return false;
                    }
                }

                true
            }

            (Ty::Tuple(lhs), Ty::Tuple(rhs)) => match (&lhs.inner, &rhs.inner) {
                (TyTupleDescriptor::Nil, TyTupleDescriptor::Nil) => true,

                (
                    TyTupleDescriptor::Cons { head: lhs_head, tail: lhs_tail, .. },
                    TyTupleDescriptor::Cons { head: rhs_head, tail: rhs_tail, .. }
                ) => {
                    self.is_ty_equivalent(lhs_head, rhs_head) 
                    && lhs_tail.value_separator_pairs.iter().zip(rhs_tail.value_separator_pairs.iter()).all(|(lhs, rhs)| {
                        self.is_ty_equivalent(&lhs.0, &rhs.0)
                    })
                    && lhs_tail.final_value_opt.as_ref().map(|lhs| {
                        rhs_tail.final_value_opt.as_ref().map(|rhs| {
                            self.is_ty_equivalent(lhs, rhs)
                        }).unwrap_or(false)
                    }).unwrap_or(false)
                }

                _ => false,
            }

            (Ty::Array(lhs), Ty::Array(rhs)) => {
                self.is_expr_equivalent(lhs.inner.length.as_ref(), rhs.inner.length.as_ref())
                && self.is_ty_equivalent(lhs.inner.ty.as_ref(), rhs.inner.ty.as_ref())
            }

            (Ty::StringSlice(_), Ty::StringSlice(_)) => true,

            (Ty::StringArray { length: lhs_length, .. }, Ty::StringArray { length: rhs_length, .. }) => {
                self.is_expr_equivalent(lhs_length.inner.as_ref(), rhs_length.inner.as_ref())
            }

            (Ty::Infer { .. }, Ty::Infer { .. }) => true,

            (Ty::Ptr { ty: lhs_ty, .. }, Ty::Ptr { ty: rhs_ty, .. }) => {
                self.is_ty_equivalent(lhs_ty.inner.as_ref(), rhs_ty.inner.as_ref())
            }

            (Ty::Slice { ty: lhs_ty, .. }, Ty::Slice { ty: rhs_ty, .. }) => {
                self.is_ty_equivalent(lhs_ty.inner.as_ref(), rhs_ty.inner.as_ref())
            }

            (Ty::Ref { ty: lhs_ty, .. }, Ty::Ref { ty: rhs_ty, .. }) => {
                self.is_ty_equivalent(lhs_ty.as_ref(), rhs_ty.as_ref())
            }

            (Ty::Never { .. }, Ty::Never { .. }) => true,

            _ => false,
        }
    }

    fn is_expr_equivalent(&self, lhs: &Expr, rhs: &Expr) -> bool { 
        match (lhs, rhs) {
            (Expr::Path(_), Expr::Path(_)) => todo!(),

            (Expr::Literal(lhs), Expr::Literal(rhs)) => match (lhs, rhs) {
                (Literal::String(lhs), Literal::String(rhs)) => lhs.parsed == rhs.parsed,
                (Literal::Char(lhs), Literal::Char(rhs)) => lhs.parsed == rhs.parsed,
                (Literal::Int(lhs), Literal::Int(rhs)) => lhs.parsed == rhs.parsed,
                (Literal::Bool(lhs), Literal::Bool(rhs)) => lhs.kind == rhs.kind,
                _ => false,
            }

            (Expr::Not { expr: lhs_expr, .. }, Expr::Not { expr: rhs_expr, .. }) => {
                self.is_expr_equivalent(lhs_expr.as_ref(), rhs_expr.as_ref())
            }

            (Expr::Mul { lhs: lhs_lhs, rhs: lhs_rhs, .. }, Expr::Mul { lhs: rhs_lhs, rhs: rhs_rhs, .. }) |
            (Expr::Div { lhs: lhs_lhs, rhs: lhs_rhs, .. }, Expr::Div { lhs: rhs_lhs, rhs: rhs_rhs, .. }) |
            (Expr::Pow { lhs: lhs_lhs, rhs: lhs_rhs, .. }, Expr::Pow { lhs: rhs_lhs, rhs: rhs_rhs, .. }) |
            (Expr::Modulo { lhs: lhs_lhs, rhs: lhs_rhs, .. }, Expr::Modulo { lhs: rhs_lhs, rhs: rhs_rhs, .. }) |
            (Expr::Add { lhs: lhs_lhs, rhs: lhs_rhs, .. }, Expr::Add { lhs: rhs_lhs, rhs: rhs_rhs, .. }) |
            (Expr::Sub { lhs: lhs_lhs, rhs: lhs_rhs, .. }, Expr::Sub { lhs: rhs_lhs, rhs: rhs_rhs, .. }) |
            (Expr::Shl { lhs: lhs_lhs, rhs: lhs_rhs, .. }, Expr::Shl { lhs: rhs_lhs, rhs: rhs_rhs, .. }) |
            (Expr::Shr { lhs: lhs_lhs, rhs: lhs_rhs, .. }, Expr::Shr { lhs: rhs_lhs, rhs: rhs_rhs, .. }) |
            (Expr::BitAnd { lhs: lhs_lhs, rhs: lhs_rhs, .. }, Expr::BitAnd { lhs: rhs_lhs, rhs: rhs_rhs, .. }) |
            (Expr::BitXor { lhs: lhs_lhs, rhs: lhs_rhs, .. }, Expr::BitXor { lhs: rhs_lhs, rhs: rhs_rhs, .. }) |
            (Expr::BitOr { lhs: lhs_lhs, rhs: lhs_rhs, .. }, Expr::BitOr { lhs: rhs_lhs, rhs: rhs_rhs, .. }) |
            (Expr::Equal { lhs: lhs_lhs, rhs: lhs_rhs, .. }, Expr::Equal { lhs: rhs_lhs, rhs: rhs_rhs, .. }) |
            (Expr::NotEqual { lhs: lhs_lhs, rhs: lhs_rhs, .. }, Expr::NotEqual { lhs: rhs_lhs, rhs: rhs_rhs, .. }) |
            (Expr::LessThan { lhs: lhs_lhs, rhs: lhs_rhs, .. }, Expr::LessThan { lhs: rhs_lhs, rhs: rhs_rhs, .. }) |
            (Expr::GreaterThan { lhs: lhs_lhs, rhs: lhs_rhs, .. }, Expr::GreaterThan { lhs: rhs_lhs, rhs: rhs_rhs, .. }) |
            (Expr::LessThanEq { lhs: lhs_lhs, rhs: lhs_rhs, .. }, Expr::LessThanEq { lhs: rhs_lhs, rhs: rhs_rhs, .. }) |
            (Expr::GreaterThanEq { lhs: lhs_lhs, rhs: lhs_rhs, .. }, Expr::GreaterThanEq { lhs: rhs_lhs, rhs: rhs_rhs, .. }) |
            (Expr::LogicalAnd { lhs: lhs_lhs, rhs: lhs_rhs, .. }, Expr::LogicalAnd { lhs: rhs_lhs, rhs: rhs_rhs, .. }) |
            (Expr::LogicalOr { lhs: lhs_lhs, rhs: lhs_rhs, .. }, Expr::LogicalOr { lhs: rhs_lhs, rhs: rhs_rhs, .. }) => {
                self.is_expr_equivalent(lhs_lhs.as_ref(), rhs_lhs.as_ref())
                && self.is_expr_equivalent(lhs_rhs.as_ref(), rhs_rhs.as_ref())
            }

            _ => false,
        }
    }
}
