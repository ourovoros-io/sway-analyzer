use crate::{error::Error, project::Project, scope::{AstScope, AstVariableKind}};
use std::{cell::RefCell, path::Path, rc::Rc};
use sway_ast::{expr::asm::AsmFinalExpr, *};
use sway_types::{BaseIdent, Span, Spanned};

#[derive(Clone)]
pub struct ModuleContext<'a> {
    pub path: &'a Path,
    pub module: &'a Module,
}

#[derive(Clone)]
pub struct ItemContext<'a> {
    pub path: &'a Path,
    pub module: &'a Module,
    pub attributes: &'a [AttributeDecl],
    pub item: &'a ItemKind,
    pub impl_attributes: Option<&'a [AttributeDecl]>,
    pub item_impl: Option<&'a ItemImpl>,
    pub fn_attributes: Option<&'a [AttributeDecl]>,
    pub item_fn: Option<&'a ItemFn>,
    pub blocks: Vec<Span>,
    pub statement: Option<&'a Statement>,
}

#[derive(Clone)]
pub struct SubmoduleContext<'a> {
    pub path: &'a Path,
    pub module: &'a Module,
    pub item: &'a ItemKind,
    pub attributes: &'a [AttributeDecl],
    pub submodule: &'a Submodule,
}

#[derive(Clone)]
pub struct UseContext<'a> {
    pub path: &'a Path,
    pub module: &'a Module,
    pub item: &'a ItemKind,
    pub attributes: &'a [AttributeDecl],
    pub item_use: &'a ItemUse,
}

#[derive(Clone)]
pub struct StructContext<'a> {
    pub path: &'a Path,
    pub module: &'a Module,
    pub item: &'a ItemKind,
    pub struct_attributes: &'a [AttributeDecl],
    pub item_struct: &'a ItemStruct,
}

#[derive(Clone)]
pub struct StructFieldContext<'a> {
    pub path: &'a Path,
    pub module: &'a Module,
    pub item: &'a ItemKind,
    pub struct_attributes: &'a [AttributeDecl],
    pub item_struct: &'a ItemStruct,
    pub field_attributes: &'a [AttributeDecl],
    pub field: &'a TypeField,
}

#[derive(Clone)]
pub struct EnumContext<'a> {
    pub path: &'a Path,
    pub module: &'a Module,
    pub item: &'a ItemKind,
    pub attributes: &'a [AttributeDecl],
    pub item_enum: &'a ItemEnum,
}

#[derive(Clone)]
pub struct EnumFieldContext<'a> {
    pub path: &'a Path,
    pub module: &'a Module,
    pub item: &'a ItemKind,
    pub enum_attributes: &'a [AttributeDecl],
    pub item_enum: &'a ItemEnum,
    pub field_attributes: &'a [AttributeDecl],
    pub field: &'a TypeField,
}

#[derive(Clone)]
pub struct FnContext<'a> {
    pub path: &'a Path,
    pub module: &'a Module,
    pub item: &'a ItemKind,
    pub impl_attributes: Option<&'a [AttributeDecl]>,
    pub item_impl: Option<&'a ItemImpl>,
    pub fn_attributes: &'a [AttributeDecl],
    pub item_fn: &'a ItemFn,
}

#[derive(Clone)]
pub struct StatementContext<'a> {
    pub path: &'a Path,
    pub module: &'a Module,
    pub item: &'a ItemKind,
    pub impl_attributes: Option<&'a [AttributeDecl]>,
    pub item_impl: Option<&'a ItemImpl>,
    pub fn_attributes: &'a [AttributeDecl],
    pub item_fn: &'a ItemFn,
    pub blocks: Vec<Span>,
    pub statement: &'a Statement,
}

#[derive(Clone)]
pub struct StatementLetContext<'a> {
    pub path: &'a Path,
    pub module: &'a Module,
    pub item: &'a ItemKind,
    pub impl_attributes: Option<&'a [AttributeDecl]>,
    pub item_impl: Option<&'a ItemImpl>,
    pub fn_attributes: &'a [AttributeDecl],
    pub item_fn: &'a ItemFn,
    pub blocks: Vec<Span>,
    pub statement: &'a Statement,
    pub statement_let: &'a StatementLet,
}

#[derive(Clone)]
pub struct ExprContext<'a> {
    pub path: &'a Path,
    pub module: &'a Module,
    pub item: &'a ItemKind,
    pub impl_attributes: Option<&'a [AttributeDecl]>,
    pub item_impl: Option<&'a ItemImpl>,
    pub fn_attributes: Option<&'a [AttributeDecl]>,
    pub item_fn: Option<&'a ItemFn>,
    pub blocks: Vec<Span>,
    pub statement: Option<&'a Statement>,
    pub expr: &'a Expr,
}

#[derive(Clone)]
pub struct BlockContext<'a> {
    pub path: &'a Path,
    pub module: &'a Module,
    pub item: &'a ItemKind,
    pub impl_attributes: Option<&'a [AttributeDecl]>,
    pub item_impl: Option<&'a ItemImpl>,
    pub fn_attributes: &'a [AttributeDecl],
    pub item_fn: &'a ItemFn,
    pub expr: Option<&'a Expr>,
    pub blocks: Vec<Span>,
    pub statement: Option<&'a Statement>,
    pub block: &'a Braces<CodeBlockContents>,
}

#[derive(Clone)]
pub struct AsmBlockContext<'a> {
    pub path: &'a Path,
    pub module: &'a Module,
    pub item: &'a ItemKind,
    pub impl_attributes: Option<&'a [AttributeDecl]>,
    pub item_impl: Option<&'a ItemImpl>,
    pub fn_attributes: &'a [AttributeDecl],
    pub item_fn: &'a ItemFn,
    pub blocks: Vec<Span>,
    pub statement: Option<&'a Statement>,
    pub expr: &'a Expr,
    pub asm: &'a AsmBlock,
}

#[derive(Clone)]
pub struct AsmInstructionContext<'a> {
    pub path: &'a Path,
    pub module: &'a Module,
    pub item: &'a ItemKind,
    pub impl_attributes: Option<&'a [AttributeDecl]>,
    pub item_impl: Option<&'a ItemImpl>,
    pub fn_attributes: &'a [AttributeDecl],
    pub item_fn: &'a ItemFn,
    pub blocks: Vec<Span>,
    pub statement: Option<&'a Statement>,
    pub expr: &'a Expr,
    pub asm: &'a AsmBlock,
    pub instruction: &'a Instruction,
}

#[derive(Clone)]
pub struct AsmFinalExprContext<'a> {
    pub path: &'a Path,
    pub module: &'a Module,
    pub item: &'a ItemKind,
    pub impl_attributes: Option<&'a [AttributeDecl]>,
    pub item_impl: Option<&'a ItemImpl>,
    pub fn_attributes: &'a [AttributeDecl],
    pub item_fn: &'a ItemFn,
    pub blocks: Vec<Span>,
    pub statement: Option<&'a Statement>,
    pub expr: &'a Expr,
    pub asm: &'a AsmBlock,
    pub final_expr: &'a AsmFinalExpr,
}

#[derive(Clone)]
pub struct IfExprContext<'a> {
    pub path: &'a Path,
    pub module: &'a Module,
    pub item: &'a ItemKind,
    pub impl_attributes: Option<&'a [AttributeDecl]>,
    pub item_impl: Option<&'a ItemImpl>,
    pub fn_attributes: &'a [AttributeDecl],
    pub item_fn: &'a ItemFn,
    pub blocks: Vec<Span>,
    pub statement: Option<&'a Statement>,
    pub expr: &'a Expr,
    pub if_expr: &'a IfExpr,
}

#[derive(Clone)]
pub struct MatchExprContext<'a> {
    pub path: &'a Path,
    pub module: &'a Module,
    pub item: &'a ItemKind,
    pub impl_attributes: Option<&'a [AttributeDecl]>,
    pub item_impl: Option<&'a ItemImpl>,
    pub fn_attributes: &'a [AttributeDecl],
    pub item_fn: &'a ItemFn,
    pub blocks: Vec<Span>,
    pub statement: Option<&'a Statement>,
    pub expr: &'a Expr,
    pub value: &'a Expr,
    pub branches: &'a Braces<Vec<MatchBranch>>,
}

#[derive(Clone)]
pub struct MatchBranchContext<'a> {
    pub path: &'a Path,
    pub module: &'a Module,
    pub item: &'a ItemKind,
    pub impl_attributes: Option<&'a [AttributeDecl]>,
    pub item_impl: Option<&'a ItemImpl>,
    pub fn_attributes: &'a [AttributeDecl],
    pub item_fn: &'a ItemFn,
    pub blocks: Vec<Span>,
    pub statement: Option<&'a Statement>,
    pub expr: &'a Expr,
    pub value: &'a Expr,
    pub branch: &'a MatchBranch,
}

#[derive(Clone)]
pub struct WhileExprContext<'a> {
    pub path: &'a Path,
    pub module: &'a Module,
    pub item: &'a ItemKind,
    pub impl_attributes: Option<&'a [AttributeDecl]>,
    pub item_impl: Option<&'a ItemImpl>,
    pub fn_attributes: &'a [AttributeDecl],
    pub item_fn: &'a ItemFn,
    pub blocks: Vec<Span>,
    pub statement: Option<&'a Statement>,
    pub expr: &'a Expr,
    pub condition: &'a Expr,
    pub body: &'a Braces<CodeBlockContents>,
}

#[derive(Clone)]
pub struct ForExprContext<'a> {
    pub path: &'a Path,
    pub module: &'a Module,
    pub item: &'a ItemKind,
    pub impl_attributes: Option<&'a [AttributeDecl]>,
    pub item_impl: Option<&'a ItemImpl>,
    pub fn_attributes: &'a [AttributeDecl],
    pub item_fn: &'a ItemFn,
    pub blocks: Vec<Span>,
    pub statement: Option<&'a Statement>,
    pub expr: &'a Expr,
    pub pattern: &'a Pattern,
    pub iterator: &'a Expr,
    pub body: &'a Braces<CodeBlockContents>,
}

#[derive(Clone)]
pub struct TraitContext<'a> {
    pub path: &'a Path,
    pub module: &'a Module,
    pub item: &'a ItemKind,
    pub attributes: &'a [AttributeDecl],
    pub item_trait: &'a ItemTrait,
}

#[derive(Clone)]
pub struct ImplContext<'a> {
    pub path: &'a Path,
    pub module: &'a Module,
    pub item: &'a ItemKind,
    pub attributes: &'a [AttributeDecl],
    pub item_impl: &'a ItemImpl,
    pub blocks: Vec<Span>,
    pub statement: Option<&'a Statement>,
}

#[derive(Clone)]
pub struct AbiContext<'a> {
    pub path: &'a Path,
    pub module: &'a Module,
    pub item: &'a ItemKind,
    pub attributes: &'a [AttributeDecl],
    pub item_abi: &'a ItemAbi,
}

#[derive(Clone)]
pub struct ConstContext<'a> {
    pub path: &'a Path,
    pub module: &'a Module,
    pub item: &'a ItemKind,
    pub impl_attributes: Option<&'a [AttributeDecl]>,
    pub item_impl: Option<&'a ItemImpl>,
    pub fn_attributes: Option<&'a [AttributeDecl]>,
    pub item_fn: Option<&'a ItemFn>,
    pub const_attributes: &'a [AttributeDecl],
    pub item_const: &'a ItemConst,
    pub blocks: Vec<Span>,
    pub statement: Option<&'a Statement>,
}

#[derive(Clone)]
pub struct StorageContext<'a> {
    pub path: &'a Path,
    pub module: &'a Module,
    pub item: &'a ItemKind,
    pub attributes: &'a [AttributeDecl],
    pub item_storage: &'a ItemStorage,
}

#[derive(Clone)]
pub struct StorageFieldContext<'a> {
    pub path: &'a Path,
    pub module: &'a Module,
    pub item: &'a ItemKind,
    pub storage_attributes: &'a [AttributeDecl],
    pub item_storage: &'a ItemStorage,
    pub field_attributes: &'a [AttributeDecl],
    pub field: &'a StorageField,
}

#[derive(Clone)]
pub struct ConfigurableContext<'a> {
    pub path: &'a Path,
    pub module: &'a Module,
    pub item: &'a ItemKind,
    pub attributes: &'a [AttributeDecl],
    pub item_configurable: &'a ItemConfigurable,
}

#[derive(Clone)]
pub struct ConfigurableFieldContext<'a> {
    pub path: &'a Path,
    pub module: &'a Module,
    pub item: &'a ItemKind,
    pub configurable_attributes: &'a [AttributeDecl],
    pub item_configurable: &'a ItemConfigurable,
    pub field_attributes: &'a [AttributeDecl],
    pub field: &'a ConfigurableField,
}

#[derive(Clone)]
pub struct TypeAliasContext<'a> {
    pub path: &'a Path,
    pub module: &'a Module,
    pub item: &'a ItemKind,
    pub attributes: &'a [AttributeDecl],
    pub item_type_alias: &'a ItemTypeAlias,
}

#[derive(Clone)]
pub struct TraitTypeContext<'a> {
    pub path: &'a Path,
    pub module: &'a Module,
    pub item: &'a ItemKind,
    pub attributes: &'a [AttributeDecl],
    pub item_type: &'a TraitType,
}

#[allow(unused_variables)]
pub trait AstVisitor {
    fn visit_module(&mut self, context: &ModuleContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_module(&mut self, context: &ModuleContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_module_item(&mut self, context: &ItemContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_module_item(&mut self, context: &ItemContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_submodule(&mut self, context: &SubmoduleContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_submodule(&mut self, context: &SubmoduleContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_use(&mut self, context: &UseContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_use(&mut self, context: &UseContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_struct(&mut self, context: &StructContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_struct(&mut self, context: &StructContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_struct_field(&mut self, context: &StructFieldContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_struct_field(&mut self, context: &StructFieldContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_enum(&mut self, context: &EnumContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_enum(&mut self, context: &EnumContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_enum_field(&mut self, context: &EnumFieldContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_enum_field(&mut self, context: &EnumFieldContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_fn(&mut self, context: &FnContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_fn(&mut self, context: &FnContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_statement(&mut self, context: &StatementContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_statement(&mut self, context: &StatementContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_statement_let(&mut self, context: &StatementLetContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_statement_let(&mut self, context: &StatementLetContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_expr(&mut self, context: &ExprContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_expr(&mut self, context: &ExprContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_block(&mut self, context: &BlockContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_block(&mut self, context: &BlockContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_asm_block(&mut self, context: &AsmBlockContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_asm_block(&mut self, context: &AsmBlockContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_asm_instruction(&mut self, context: &AsmInstructionContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_asm_instruction(&mut self, context: &AsmInstructionContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_asm_final_expr(&mut self, context: &AsmFinalExprContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_asm_final_expr(&mut self, context: &AsmFinalExprContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_if_expr(&mut self, context: &IfExprContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_if_expr(&mut self, context: &IfExprContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_match_expr(&mut self, context: &MatchExprContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_match_expr(&mut self, context: &MatchExprContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_match_branch(&mut self, context: &MatchBranchContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_match_branch(&mut self, context: &MatchBranchContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_while_expr(&mut self, context: &WhileExprContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_while_expr(&mut self, context: &WhileExprContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_for_expr(&mut self, context: &ForExprContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_for_expr(&mut self, context: &ForExprContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_trait(&mut self, context: &TraitContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_trait(&mut self, context: &TraitContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_impl(&mut self, context: &ImplContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_impl(&mut self, context: &ImplContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_abi(&mut self, context: &AbiContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_abi(&mut self, context: &AbiContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_const(&mut self, context: &ConstContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_const(&mut self, context: &ConstContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_storage(&mut self, context: &StorageContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_storage(&mut self, context: &StorageContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_storage_field(&mut self, context: &StorageFieldContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_storage_field(&mut self, context: &StorageFieldContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_configurable(&mut self, context: &ConfigurableContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_configurable(&mut self, context: &ConfigurableContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_configurable_field(&mut self, context: &ConfigurableFieldContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_configurable_field(&mut self, context: &ConfigurableFieldContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_type_alias(&mut self, context: &TypeAliasContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_type_alias(&mut self, context: &TypeAliasContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_trait_type(&mut self, context: &TraitTypeContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_trait_type(&mut self, context: &TraitTypeContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> { Ok(()) }
}

#[derive(Default)]
pub struct AstVisitorRecursive<'a> {
    pub visitors: Vec<Box<dyn AstVisitor>>,
    pub visit_module_hooks: Vec<Box<dyn FnMut(&ModuleContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub leave_module_hooks: Vec<Box<dyn FnMut(&ModuleContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub visit_module_item_hooks: Vec<Box<dyn FnMut(&ItemContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub leave_module_item_hooks: Vec<Box<dyn FnMut(&ItemContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub visit_submodule_hooks: Vec<Box<dyn FnMut(&SubmoduleContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub leave_submodule_hooks: Vec<Box<dyn FnMut(&SubmoduleContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub visit_use_hooks: Vec<Box<dyn FnMut(&UseContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub leave_use_hooks: Vec<Box<dyn FnMut(&UseContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub visit_struct_hooks: Vec<Box<dyn FnMut(&StructContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub leave_struct_hooks: Vec<Box<dyn FnMut(&StructContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub visit_struct_field_hooks: Vec<Box<dyn FnMut(&StructFieldContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub leave_struct_field_hooks: Vec<Box<dyn FnMut(&StructFieldContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub visit_enum_hooks: Vec<Box<dyn FnMut(&EnumContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub leave_enum_hooks: Vec<Box<dyn FnMut(&EnumContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub visit_enum_field_hooks: Vec<Box<dyn FnMut(&EnumFieldContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub leave_enum_field_hooks: Vec<Box<dyn FnMut(&EnumFieldContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub visit_fn_hooks: Vec<Box<dyn FnMut(&FnContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub leave_fn_hooks: Vec<Box<dyn FnMut(&FnContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub visit_statement_hooks: Vec<Box<dyn FnMut(&StatementContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub leave_statement_hooks: Vec<Box<dyn FnMut(&StatementContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub visit_statement_let_hooks: Vec<Box<dyn FnMut(&StatementLetContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub leave_statement_let_hooks: Vec<Box<dyn FnMut(&StatementLetContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub visit_expr_hooks: Vec<Box<dyn FnMut(&ExprContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub leave_expr_hooks: Vec<Box<dyn FnMut(&ExprContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub visit_block_hooks: Vec<Box<dyn FnMut(&BlockContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub leave_block_hooks: Vec<Box<dyn FnMut(&BlockContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub visit_asm_block_hooks: Vec<Box<dyn FnMut(&AsmBlockContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub leave_asm_block_hooks: Vec<Box<dyn FnMut(&AsmBlockContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub visit_asm_instruction_hooks: Vec<Box<dyn FnMut(&AsmInstructionContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub leave_asm_instruction_hooks: Vec<Box<dyn FnMut(&AsmInstructionContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub visit_asm_final_expr_hooks: Vec<Box<dyn FnMut(&AsmFinalExprContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub leave_asm_final_expr_hooks: Vec<Box<dyn FnMut(&AsmFinalExprContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub visit_if_expr_hooks: Vec<Box<dyn FnMut(&IfExprContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub leave_if_expr_hooks: Vec<Box<dyn FnMut(&IfExprContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub visit_match_expr_hooks: Vec<Box<dyn FnMut(&MatchExprContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub leave_match_expr_hooks: Vec<Box<dyn FnMut(&MatchExprContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub visit_match_branch_hooks: Vec<Box<dyn FnMut(&MatchBranchContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub leave_match_branch_hooks: Vec<Box<dyn FnMut(&MatchBranchContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub visit_while_expr_hooks: Vec<Box<dyn FnMut(&WhileExprContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub leave_while_expr_hooks: Vec<Box<dyn FnMut(&WhileExprContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub visit_for_expr_hooks: Vec<Box<dyn FnMut(&ForExprContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub leave_for_expr_hooks: Vec<Box<dyn FnMut(&ForExprContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub visit_trait_hooks: Vec<Box<dyn FnMut(&TraitContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub leave_trait_hooks: Vec<Box<dyn FnMut(&TraitContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub visit_impl_hooks: Vec<Box<dyn FnMut(&ImplContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub leave_impl_hooks: Vec<Box<dyn FnMut(&ImplContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub visit_abi_hooks: Vec<Box<dyn FnMut(&AbiContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub leave_abi_hooks: Vec<Box<dyn FnMut(&AbiContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub visit_const_hooks: Vec<Box<dyn FnMut(&ConstContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub leave_const_hooks: Vec<Box<dyn FnMut(&ConstContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub visit_storage_hooks: Vec<Box<dyn FnMut(&StorageContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub leave_storage_hooks: Vec<Box<dyn FnMut(&StorageContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub visit_storage_field_hooks: Vec<Box<dyn FnMut(&StorageFieldContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub leave_storage_field_hooks: Vec<Box<dyn FnMut(&StorageFieldContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub visit_configurable_hooks: Vec<Box<dyn FnMut(&ConfigurableContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub leave_configurable_hooks: Vec<Box<dyn FnMut(&ConfigurableContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub visit_configurable_field_hooks: Vec<Box<dyn FnMut(&ConfigurableFieldContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub leave_configurable_field_hooks: Vec<Box<dyn FnMut(&ConfigurableFieldContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub visit_type_alias_hooks: Vec<Box<dyn FnMut(&TypeAliasContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub leave_type_alias_hooks: Vec<Box<dyn FnMut(&TypeAliasContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub visit_trait_type_hooks: Vec<Box<dyn FnMut(&TraitTypeContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
    pub leave_trait_type_hooks: Vec<Box<dyn FnMut(&TraitTypeContext, Rc<RefCell<AstScope>>, &mut Project) -> Result<(), Error> + 'a>>,
}

impl AstVisitor for AstVisitorRecursive<'_> {
    fn visit_module(&mut self, context: &ModuleContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_module(context, scope.clone(), project)?;
        }

        for hook in self.visit_module_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        for item in context.module.items.iter() {
            let context = ItemContext {
                path: context.path,
                module: context.module,
                attributes: item.attributes.as_slice(),
                item: &item.value,
                impl_attributes: None,
                item_impl: None,
                fn_attributes: None,
                item_fn: None,
                blocks: vec![],
                statement: None,
            };
            
            self.visit_module_item(&context, scope.clone(), project)?;
            self.leave_module_item(&context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn leave_module(&mut self, context: &ModuleContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_module(context, scope.clone(), project)?;
        }

        for hook in self.leave_module_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn visit_module_item(&mut self, context: &ItemContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_module_item(context, scope.clone(), project)?;
        }

        for hook in self.visit_module_item_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        match &context.item {
            ItemKind::Submodule(submodule) => {
                let context = SubmoduleContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    attributes: context.attributes,
                    submodule,
                };
                
                self.visit_submodule(&context, scope.clone(), project)?;
                self.leave_submodule(&context, scope.clone(), project)?;
            }

            ItemKind::Use(item_use) => {
                let context = UseContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    attributes: context.attributes,
                    item_use,
                };
                
                self.visit_use(&context, scope.clone(), project)?;
                self.leave_use(&context, scope.clone(), project)?;
            }

            ItemKind::Struct(item_struct) => {
                let context = StructContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    struct_attributes: context.attributes,
                    item_struct,
                };
                
                self.visit_struct(&context, scope.clone(), project)?;
                self.leave_struct(&context, scope.clone(), project)?;
            }

            ItemKind::Enum(item_enum) => {
                let context = EnumContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    attributes: context.attributes,
                    item_enum,
                };
                
                self.visit_enum(&context, scope.clone(), project)?;
                self.leave_enum(&context, scope.clone(), project)?;
            }

            ItemKind::Fn(item_fn) => {
                let context = FnContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: None,
                    item_impl: None,
                    fn_attributes: context.attributes,
                    item_fn,
                };
                
                self.visit_fn(&context, scope.clone(), project)?;
                self.leave_fn(&context, scope.clone(), project)?;
            }

            ItemKind::Trait(item_trait) => {
                let context = TraitContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    attributes: context.attributes,
                    item_trait,
                };
                
                self.visit_trait(&context, scope.clone(), project)?;
                self.leave_trait(&context, scope.clone(), project)?;
            }

            ItemKind::Impl(item_impl) => {
                let context = ImplContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    attributes: context.attributes,
                    item_impl,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                };
                
                self.visit_impl(&context, scope.clone(), project)?;
                self.leave_impl(&context, scope.clone(), project)?;
            }

            ItemKind::Abi(item_abi) => {
                let context = AbiContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    attributes: context.attributes,
                    item_abi,
                };
                
                self.visit_abi(&context, scope.clone(), project)?;
                self.leave_abi(&context, scope.clone(), project)?;
            }

            ItemKind::Const(item_const) => {
                let context = ConstContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: None,
                    item_impl: None,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    const_attributes: context.attributes,
                    item_const,
                    blocks: vec![],
                    statement: None,
                };
                
                self.visit_const(&context, scope.clone(), project)?;
                self.leave_const(&context, scope.clone(), project)?;
            }

            ItemKind::Storage(item_storage) => {
                let context = StorageContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    attributes: context.attributes,
                    item_storage,
                };
                
                self.visit_storage(&context, scope.clone(), project)?;
                self.leave_storage(&context, scope.clone(), project)?;
            }

            ItemKind::Configurable(item_configurable) => {
                let context = ConfigurableContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    attributes: context.attributes,
                    item_configurable,
                };
                
                self.visit_configurable(&context, scope.clone(), project)?;
                self.leave_configurable(&context, scope.clone(), project)?;
            }

            ItemKind::TypeAlias(item_type_alias) => {
                let context = TypeAliasContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    attributes: context.attributes,
                    item_type_alias,
                };
                
                self.visit_type_alias(&context, scope.clone(), project)?;
                self.leave_type_alias(&context, scope.clone(), project)?;
            }

            ItemKind::Error(_, _) => {}
        }

        Ok(())
    }

    fn leave_module_item(&mut self, context: &ItemContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_module_item(context, scope.clone(), project)?;
        }

        for hook in self.leave_module_item_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn visit_submodule(&mut self, context: &SubmoduleContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_submodule(context, scope.clone(), project)?;
        }

        for hook in self.visit_submodule_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn leave_submodule(&mut self, context: &SubmoduleContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_submodule(context, scope.clone(), project)?;
        }
        
        for hook in self.leave_submodule_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn visit_use(&mut self, context: &UseContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        // scope.borrow_mut().add_use(context.item_use);
        
        for visitor in self.visitors.iter_mut() {
            visitor.visit_use(context, scope.clone(), project)?;
        }
        
        for hook in self.visit_use_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn leave_use(&mut self, context: &UseContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_use(context, scope.clone(), project)?;
        }
        
        for hook in self.leave_use_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn visit_struct(&mut self, context: &StructContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        // scope.borrow_mut().add_struct(project, context.item_struct);

        let scope = Rc::new(RefCell::new(AstScope::new(Some(scope.clone()))));
        
        // if let Some(generics) = context.item_struct.generics.as_ref() {
        //     scope.borrow_mut().add_generic_params(project, generics, context.item_struct.where_clause_opt.as_ref());
        // }
        
        for visitor in self.visitors.iter_mut() {
            visitor.visit_struct(context, scope.clone(), project)?;
        }
        
        for hook in self.visit_struct_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        for field in &context.item_struct.fields.inner {
            let context = StructFieldContext {
                path: context.path,
                module: context.module,
                item: context.item,
                struct_attributes: context.struct_attributes,
                item_struct: context.item_struct,
                field_attributes: field.attributes.as_slice(),
                field: &field.value,
            };

            self.visit_struct_field(&context, scope.clone(), project)?;
            self.leave_struct_field(&context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn leave_struct(&mut self, context: &StructContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_struct(context, scope.clone(), project)?;
        }
        
        for hook in self.leave_struct_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn visit_struct_field(&mut self, context: &StructFieldContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_struct_field(context, scope.clone(), project)?;
        }

        for hook in self.visit_struct_field_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn leave_struct_field(&mut self, context: &StructFieldContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_struct_field(context, scope.clone(), project)?;
        }

        for hook in self.leave_struct_field_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn visit_enum(&mut self, context: &EnumContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        // scope.borrow_mut().add_enum(project, context.item_enum);

        let scope = Rc::new(RefCell::new(AstScope::new(Some(scope.clone()))));
        
        // if let Some(generics) = context.item_enum.generics.as_ref() {
        //     scope.borrow_mut().add_generic_params(project, generics, context.item_enum.where_clause_opt.as_ref());
        // }
        
        for visitor in self.visitors.iter_mut() {
            visitor.visit_enum(context, scope.clone(), project)?;
        }
        
        for hook in self.visit_enum_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        for field in &context.item_enum.fields.inner {
            let context = EnumFieldContext {
                path: context.path,
                module: context.module,
                item: context.item,
                enum_attributes: context.attributes,
                item_enum: context.item_enum,
                field_attributes: field.attributes.as_slice(),
                field: &field.value,
            };

            self.visit_enum_field(&context, scope.clone(), project)?;
            self.leave_enum_field(&context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn leave_enum(&mut self, context: &EnumContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_enum(context, scope.clone(), project)?;
        }
        
        for hook in self.leave_enum_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn visit_enum_field(&mut self, context: &EnumFieldContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_enum_field(context, scope.clone(), project)?;
        }
        
        for hook in self.visit_enum_field_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn leave_enum_field(&mut self, context: &EnumFieldContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_enum_field(context, scope.clone(), project)?;
        }
        
        for hook in self.leave_enum_field_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn visit_fn(&mut self, context: &FnContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        // scope.borrow_mut().add_fn_signature(project, &context.item_fn.fn_signature);
        
        let scope = Rc::new(RefCell::new(AstScope::new(Some(scope.clone()))));
        
        // if let Some(generics) = context.item_fn.fn_signature.generics.as_ref() {
        //     scope.borrow_mut().add_generic_params(project, generics, context.item_fn.fn_signature.where_clause_opt.as_ref());
        // }

        let args = match &context.item_fn.fn_signature.arguments.inner {
            FnArgs::Static(args) => Some(args),

            FnArgs::NonStatic { args_opt, .. } => {
                // scope.borrow_mut().add_variable(
                //     project,
                //     AstVariableKind::Parameter,
                //     &BaseIdent::new_no_span("self".into()),
                //     &context.item_impl.as_ref().unwrap().ty,
                // );

                args_opt.as_ref().map(|(_, args)| args)
            }
        };

        // if let Some(args) = args {
        //     for arg in args {
        //         crate::utils::map_pattern_and_ty(&arg.pattern, &arg.ty, &mut |pattern, ty| {
        //             match pattern {
        //                 Pattern::Var { name, .. } => {
        //                     scope.borrow_mut().add_variable(project, AstVariableKind::Parameter, name, ty);
        //                 }

        //                 _ => {}
        //             }
        //         });
        //     }
        // }

        for visitor in self.visitors.iter_mut() {
            visitor.visit_fn(context, scope.clone(), project)?;
        }

        for hook in self.visit_fn_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        let body_context = BlockContext {
            path: context.path,
            module: context.module,
            item: context.item,
            impl_attributes: context.impl_attributes,
            item_impl: context.item_impl,
            fn_attributes: context.fn_attributes,
            item_fn: context.item_fn,
            expr: None,
            blocks: vec![],
            statement: None,
            block: &context.item_fn.body,
        };

        self.visit_block(&body_context, scope.clone(), project)?;
        self.leave_block(&body_context, scope.clone(), project)?;
        
        Ok(())
    }

    fn leave_fn(&mut self, context: &FnContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_fn(context, scope.clone(), project)?;
        }
        
        for hook in self.leave_fn_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn visit_statement(&mut self, context: &StatementContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_statement(context, scope.clone(), project)?;
        }
        
        for hook in self.visit_statement_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        match context.statement {
            Statement::Let(statement_let) => {
                let context = StatementLetContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    statement_let,
                };

                self.visit_statement_let(&context, scope.clone(), project)?;
                self.leave_statement_let(&context, scope.clone(), project)?;
            }

            Statement::Item(item) => {
                let context = ItemContext {
                    path: context.path,
                    module: context.module,
                    attributes: item.attributes.as_slice(),
                    item: &item.value,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: Some(context.fn_attributes),
                    item_fn: Some(context.item_fn),
                    blocks: context.blocks.clone(),
                    statement: Some(context.statement),
                };

                self.visit_module_item(&context, scope.clone(), project)?;
                self.leave_module_item(&context, scope.clone(), project)?;
            }

            Statement::Expr { expr, .. } => {
                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: Some(context.fn_attributes),
                    item_fn: Some(context.item_fn),
                    blocks: context.blocks.clone(),
                    statement: Some(context.statement),
                    expr,
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;
            }

            Statement::Error(_, _) => {}
        }

        Ok(())
    }

    fn leave_statement(&mut self, context: &StatementContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_statement(context, scope.clone(), project)?;
        }
        
        for hook in self.leave_statement_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn visit_statement_let(&mut self, context: &StatementLetContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        // crate::utils::map_pattern_and_ty(
        //     &context.statement_let.pattern,
        //     &context.statement_let.ty_opt.as_ref()
        //         .map(|(_, ty)| ty.clone())
        //         .unwrap_or_else(|| scope.borrow().get_expr_ty(&context.statement_let.expr, project)),
        //     &mut |pattern, ty| {
        //         match pattern {
        //             Pattern::Var { name, .. } => {
        //                 scope.borrow_mut().add_variable(project, AstVariableKind::Local, name, ty);
        //             }

        //             _ => {}
        //         }
        //     },
        // );

        for visitor in self.visitors.iter_mut() {
            visitor.visit_statement_let(context, scope.clone(), project)?;
        }
        
        for hook in self.visit_statement_let_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        let context = ExprContext {
            path: context.path,
            module: context.module,
            item: context.item,
            impl_attributes: context.impl_attributes,
            item_impl: context.item_impl,
            fn_attributes: Some(context.fn_attributes),
            item_fn: Some(context.item_fn),
            blocks: context.blocks.clone(),
            statement: Some(context.statement),
            expr: &context.statement_let.expr,
        };

        self.visit_expr(&context, scope.clone(), project)?;
        self.leave_expr(&context, scope.clone(), project)?;

        Ok(())
    }

    fn leave_statement_let(&mut self, context: &StatementLetContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_statement_let(context, scope.clone(), project)?;
        }
        
        for hook in self.leave_statement_let_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn visit_expr(&mut self, context: &ExprContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_expr(context, scope.clone(), project)?;
        }

        for hook in self.visit_expr_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        match context.expr {
            Expr::Error(_, _) => {}
            Expr::Path(_) => {}
            Expr::Literal(_) => {}

            Expr::AbiCast { args, .. } => {
                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: args.inner.address.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;
            }

            Expr::Struct { fields, .. } => {
                for field in &fields.inner {
                    if let Some(field) = field.expr_opt.as_ref() {
                        let context = ExprContext {
                            path: context.path,
                            module: context.module,
                            item: context.item,
                            impl_attributes: context.impl_attributes,
                            item_impl: context.item_impl,
                            fn_attributes: context.fn_attributes,
                            item_fn: context.item_fn,
                            blocks: context.blocks.clone(),
                            statement: context.statement,
                            expr: field.1.as_ref(),
                        };
        
                        self.visit_expr(&context, scope.clone(), project)?;
                        self.leave_expr(&context, scope.clone(), project)?;
                    }
                }
            }

            Expr::Tuple(tuple) => {
                match &tuple.inner {
                    ExprTupleDescriptor::Nil => {}

                    ExprTupleDescriptor::Cons { head, tail, .. } => {
                        let context = ExprContext {
                            path: context.path,
                            module: context.module,
                            item: context.item,
                            impl_attributes: context.impl_attributes,
                            item_impl: context.item_impl,
                            fn_attributes: context.fn_attributes,
                            item_fn: context.item_fn,
                            blocks: context.blocks.clone(),
                            statement: context.statement,
                            expr: head.as_ref(),
                        };
        
                        self.visit_expr(&context, scope.clone(), project)?;
                        self.leave_expr(&context, scope.clone(), project)?;

                        for expr in tail {
                            let context = ExprContext {
                                path: context.path,
                                module: context.module,
                                item: context.item,
                                impl_attributes: context.impl_attributes,
                                item_impl: context.item_impl,
                                fn_attributes: context.fn_attributes,
                                item_fn: context.item_fn,
                                blocks: context.blocks.clone(),
                                statement: context.statement,
                                expr,
                            };
            
                            self.visit_expr(&context, scope.clone(), project)?;
                            self.leave_expr(&context, scope.clone(), project)?;
                        }
                    }
                }
            }
            Expr::Parens(parens) => {
                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: parens.inner.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;
            }
            
            Expr::Block(block) => {
                let context = BlockContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes.unwrap(),
                    item_fn: context.item_fn.unwrap(),
                    expr: Some(context.expr),
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    block,
                };

                let scope = Rc::new(RefCell::new(AstScope::new(Some(scope.clone()))));
                
                self.visit_block(&context, scope.clone(), project)?;
                self.leave_block(&context, scope.clone(), project)?;
            }
            
            Expr::Array(array) => {
                match &array.inner {
                    ExprArrayDescriptor::Sequence(sequence) => {
                        for expr in sequence {
                            let context = ExprContext {
                                path: context.path,
                                module: context.module,
                                item: context.item,
                                impl_attributes: context.impl_attributes,
                                item_impl: context.item_impl,
                                fn_attributes: context.fn_attributes,
                                item_fn: context.item_fn,
                                blocks: context.blocks.clone(),
                                statement: context.statement,
                                expr,
                            };
            
                            self.visit_expr(&context, scope.clone(), project)?;
                            self.leave_expr(&context, scope.clone(), project)?;
                        }
                    }

                    ExprArrayDescriptor::Repeat { value, length, .. } => {
                        let context = ExprContext {
                            path: context.path,
                            module: context.module,
                            item: context.item,
                            impl_attributes: context.impl_attributes,
                            item_impl: context.item_impl,
                            fn_attributes: context.fn_attributes,
                            item_fn: context.item_fn,
                            blocks: context.blocks.clone(),
                            statement: context.statement,
                            expr: value.as_ref(),
                        };
        
                        self.visit_expr(&context, scope.clone(), project)?;
                        self.leave_expr(&context, scope.clone(), project)?;

                        let context = ExprContext {
                            path: context.path,
                            module: context.module,
                            item: context.item,
                            impl_attributes: context.impl_attributes,
                            item_impl: context.item_impl,
                            fn_attributes: context.fn_attributes,
                            item_fn: context.item_fn,
                            blocks: context.blocks.clone(),
                            statement: context.statement,
                            expr: length.as_ref(),
                        };

                        self.visit_expr(&context, scope.clone(), project)?;
                        self.leave_expr(&context, scope.clone(), project)?;
                    }
                }
            }
            
            Expr::Asm(asm) => {
                let context = AsmBlockContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes.unwrap(),
                    item_fn: context.item_fn.unwrap(),
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: context.expr,
                    asm,
                };

                self.visit_asm_block(&context, scope.clone(), project)?;
                self.leave_asm_block(&context, scope.clone(), project)?;
            }

            Expr::Return { expr_opt, .. } => {
                if let Some(expr) = expr_opt {
                    let context = ExprContext {
                        path: context.path,
                        module: context.module,
                        item: context.item,
                        impl_attributes: context.impl_attributes,
                        item_impl: context.item_impl,
                        fn_attributes: context.fn_attributes,
                        item_fn: context.item_fn,
                        blocks: context.blocks.clone(),
                        statement: context.statement,
                        expr: expr.as_ref(),
                    };
    
                    self.visit_expr(&context, scope.clone(), project)?;
                    self.leave_expr(&context, scope.clone(), project)?;
                }
            }

            Expr::If(if_expr) => {
                let context = IfExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes.unwrap(),
                    item_fn: context.item_fn.unwrap(),
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: context.expr,
                    if_expr,
                };

                self.visit_if_expr(&context, scope.clone(), project)?;
                self.leave_if_expr(&context, scope.clone(), project)?;
            }

            Expr::Match { value, branches, .. } => {
                let context = MatchExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes.unwrap(),
                    item_fn: context.item_fn.unwrap(),
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: context.expr,
                    value: value.as_ref(),
                    branches,
                };

                self.visit_match_expr(&context, scope.clone(), project)?;
                self.leave_match_expr(&context, scope.clone(), project)?;
            }

            Expr::While { condition, block, .. } => {
                let context = WhileExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes.unwrap(),
                    item_fn: context.item_fn.unwrap(),
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: context.expr,
                    condition: condition.as_ref(),
                    body: block,
                };

                self.visit_while_expr(&context, scope.clone(), project)?;
                self.leave_while_expr(&context, scope.clone(), project)?;
            }

            Expr::For { value_pattern, iterator, block, .. } => {
                let context = ForExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes.unwrap(),
                    item_fn: context.item_fn.unwrap(),
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: context.expr,
                    pattern: value_pattern,
                    iterator: iterator.as_ref(),
                    body: block,
                };

                self.visit_for_expr(&context, scope.clone(), project)?;
                self.leave_for_expr(&context, scope.clone(), project)?;
            }
            
            Expr::FuncApp { func, args } => {
                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: func.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;

                for arg in &args.inner {
                    let context = ExprContext {
                        path: context.path,
                        module: context.module,
                        item: context.item,
                        impl_attributes: context.impl_attributes,
                        item_impl: context.item_impl,
                        fn_attributes: context.fn_attributes,
                        item_fn: context.item_fn,
                        blocks: context.blocks.clone(),
                        statement: context.statement,
                        expr: arg,
                    };
    
                    self.visit_expr(&context, scope.clone(), project)?;
                    self.leave_expr(&context, scope.clone(), project)?;    
                }
            }

            Expr::Index { target, arg } => {
                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: target.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;

                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: arg.inner.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;
            }

            Expr::MethodCall { target, contract_args_opt, args, .. } => {
                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: target.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;

                if let Some(opts) = contract_args_opt.as_ref() {
                    for expr in &opts.inner {
                        if let Some(expr) = expr.expr_opt.as_ref() {
                            let context = ExprContext {
                                path: context.path,
                                module: context.module,
                                item: context.item,
                                impl_attributes: context.impl_attributes,
                                item_impl: context.item_impl,
                                fn_attributes: context.fn_attributes,
                                item_fn: context.item_fn,
                                blocks: context.blocks.clone(),
                                statement: context.statement,
                                expr: expr.1.as_ref(),
                            };
            
                            self.visit_expr(&context, scope.clone(), project)?;
                            self.leave_expr(&context, scope.clone(), project)?;
                        }
                    }
                }
            
                for arg in &args.inner {
                    let context = ExprContext {
                        path: context.path,
                        module: context.module,
                        item: context.item,
                        impl_attributes: context.impl_attributes,
                        item_impl: context.item_impl,
                        fn_attributes: context.fn_attributes,
                        item_fn: context.item_fn,
                        blocks: context.blocks.clone(),
                        statement: context.statement,
                        expr: arg,
                    };
    
                    self.visit_expr(&context, scope.clone(), project)?;
                    self.leave_expr(&context, scope.clone(), project)?;
                }
            }

            Expr::FieldProjection { target, .. } => {
                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: target.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;
            }

            Expr::TupleFieldProjection { target, .. } => {
                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: target.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;
            }

            Expr::Ref { expr, .. } => {
                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: expr.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;
            }

            Expr::Deref { expr, .. } => {
                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: expr.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;
            }

            Expr::Not { expr, .. } => {
                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: expr.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;
            }

            Expr::Mul { lhs, rhs, .. } => {
                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: lhs.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;

                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: rhs.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;
            }

            Expr::Div { lhs, rhs, .. } => {
                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: lhs.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;

                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: rhs.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;
            }

            Expr::Pow { lhs, rhs, .. } => {
                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: lhs.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;

                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: rhs.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;
            }

            Expr::Modulo { lhs, rhs, .. } => {
                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: lhs.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;

                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: rhs.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;
            }

            Expr::Add { lhs, rhs, .. } => {
                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: lhs.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;

                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: rhs.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;
            }

            Expr::Sub { lhs, rhs, .. } => {
                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: lhs.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;

                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: rhs.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;
            }

            Expr::Shl { lhs, rhs, .. } => {
                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: lhs.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;

                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: rhs.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;
            }

            Expr::Shr { lhs, rhs, .. } => {
                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: lhs.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;

                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: rhs.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;
            }

            Expr::BitAnd { lhs, rhs, .. } => {
                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: lhs.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;

                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: rhs.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;
            }

            Expr::BitXor { lhs, rhs, .. } => {
                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: lhs.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;

                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: rhs.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;
            }

            Expr::BitOr { lhs, rhs, .. } => {
                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: lhs.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;

                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: rhs.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;
            }

            Expr::Equal { lhs, rhs, .. } => {
                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: lhs.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;

                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: rhs.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;
            }

            Expr::NotEqual { lhs, rhs, .. } => {
                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: lhs.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;

                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: rhs.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;
            }

            Expr::LessThan { lhs, rhs, .. } => {
                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: lhs.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;

                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: rhs.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;
            }

            Expr::GreaterThan { lhs, rhs, .. } => {
                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: lhs.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;

                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: rhs.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;
            }

            Expr::LessThanEq { lhs, rhs, .. } => {
                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: lhs.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;

                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: rhs.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;
            }

            Expr::GreaterThanEq { lhs, rhs, .. } => {
                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: lhs.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;

                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: rhs.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;
            }

            Expr::LogicalAnd { lhs, rhs, .. } => {
                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: lhs.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;

                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: rhs.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;
            }

            Expr::LogicalOr { lhs, rhs, .. } => {
                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: lhs.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;

                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: rhs.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;
            }

            Expr::Reassignment { expr, .. } => {
                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr: expr.as_ref(),
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;
            }

            Expr::Break { .. } => {}
            Expr::Continue { .. } => {}

            Expr::Panic { expr_opt, .. } => {
                if let Some(expr) = expr_opt.as_ref() {
                    let context = ExprContext {
                        path: context.path,
                        module: context.module,
                        item: context.item,
                        impl_attributes: context.impl_attributes,
                        item_impl: context.item_impl,
                        fn_attributes: context.fn_attributes,
                        item_fn: context.item_fn,
                        blocks: context.blocks.clone(),
                        statement: context.statement,
                        expr: expr.as_ref(),
                    };
    
                    self.visit_expr(&context, scope.clone(), project)?;
                }
            }
        }
        
        Ok(())
    }

    fn leave_expr(&mut self, context: &ExprContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_expr(context, scope.clone(), project)?;
        }
        
        for hook in self.leave_expr_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn visit_block(&mut self, context: &BlockContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_block(context, scope.clone(), project)?;
        }

        for hook in self.visit_block_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        let mut blocks = context.blocks.clone();
        blocks.push(context.block.span());

        for statement in context.block.inner.statements.iter() {
            let context = StatementContext {
                path: context.path,
                module: context.module,
                item: context.item,
                impl_attributes: context.impl_attributes,
                item_impl: context.item_impl,
                fn_attributes: context.fn_attributes,
                item_fn: context.item_fn,
                blocks: blocks.clone(),
                statement,
            };

            self.visit_statement(&context, scope.clone(), project)?;
            self.leave_statement(&context, scope.clone(), project)?;
        }

        if let Some(expr) = context.block.inner.final_expr_opt.as_ref() {
            let context = ExprContext {
                path: context.path,
                module: context.module,
                item: context.item,
                impl_attributes: context.impl_attributes,
                item_impl: context.item_impl,
                fn_attributes: Some(context.fn_attributes),
                item_fn: Some(context.item_fn),
                blocks: blocks.clone(),
                statement: context.statement,
                expr: expr.as_ref(),
            };

            self.visit_expr(&context, scope.clone(), project)?;
            self.leave_expr(&context, scope.clone(), project)?;
        }
        
        Ok(())
    }

    fn leave_block(&mut self, context: &BlockContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_block(context, scope.clone(), project)?;
        }
        
        for hook in self.leave_block_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn visit_asm_block(&mut self, context: &AsmBlockContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_asm_block(context, scope.clone(), project)?;
        }

        for hook in self.visit_asm_block_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        for instruction in context.asm.contents.inner.instructions.iter() {
            let context = AsmInstructionContext {
                path: context.path,
                module: context.module,
                item: context.item,
                impl_attributes: context.impl_attributes,
                item_impl: context.item_impl,
                fn_attributes: context.fn_attributes,
                item_fn: context.item_fn,
                blocks: context.blocks.clone(),
                statement: context.statement,
                expr: context.expr,
                asm: context.asm,
                instruction: &instruction.0,
            };

            self.visit_asm_instruction(&context, scope.clone(), project)?;
            self.leave_asm_instruction(&context, scope.clone(), project)?;
        }

        if let Some(final_expr) = context.asm.contents.inner.final_expr_opt.as_ref() {
            let context = AsmFinalExprContext {
                path: context.path,
                module: context.module,
                item: context.item,
                impl_attributes: context.impl_attributes,
                item_impl: context.item_impl,
                fn_attributes: context.fn_attributes,
                item_fn: context.item_fn,
                blocks: context.blocks.clone(),
                statement: context.statement,
                expr: context.expr,
                asm: context.asm,
                final_expr,
            };

            self.visit_asm_final_expr(&context, scope.clone(), project)?;
            self.leave_asm_final_expr(&context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn leave_asm_block(&mut self, context: &AsmBlockContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_asm_block(context, scope.clone(), project)?;
        }
        
        for hook in self.leave_asm_block_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn visit_asm_instruction(&mut self, context: &AsmInstructionContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_asm_instruction(context, scope.clone(), project)?;
        }
        
        for hook in self.visit_asm_instruction_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn leave_asm_instruction(&mut self, context: &AsmInstructionContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_asm_instruction(context, scope.clone(), project)?;
        }
        
        for hook in self.leave_asm_instruction_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn visit_asm_final_expr(&mut self, context: &AsmFinalExprContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_asm_final_expr(context, scope.clone(), project)?;
        }
        
        for hook in self.visit_asm_final_expr_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn leave_asm_final_expr(&mut self, context: &AsmFinalExprContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_asm_final_expr(context, scope.clone(), project)?;
        }
        
        for hook in self.leave_asm_final_expr_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn visit_if_expr(&mut self, context: &IfExprContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_if_expr(context, scope.clone(), project)?;
        }

        for hook in self.visit_if_expr_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        {
            let parent_scope = scope.clone();
            let scope = Rc::new(RefCell::new(AstScope::new(Some(scope.clone()))));
            
            match &context.if_expr.condition {
                IfCondition::Expr(expr) => {
                    let context = ExprContext {
                        path: context.path,
                        module: context.module,
                        item: context.item,
                        impl_attributes: context.impl_attributes,
                        item_impl: context.item_impl,
                        fn_attributes: Some(context.fn_attributes),
                        item_fn: Some(context.item_fn),
                        blocks: context.blocks.clone(),
                        statement: context.statement,
                        expr: expr.as_ref(),
                    };

                    self.visit_expr(&context, parent_scope.clone(), project)?;
                    self.leave_expr(&context, parent_scope.clone(), project)?;
                }
                
                IfCondition::Let { lhs, rhs, .. } => {
                    //
                    // NOTE: `lhs` pattern can be handled by overriding `visit_if_expr`
                    //

                    // crate::utils::map_pattern_and_ty(
                    //     lhs.as_ref(),
                    //     &scope.borrow().get_expr_ty(rhs, project),
                    //     &mut |pattern, ty| {
                    //         match pattern {
                    //             Pattern::Var { name, .. } => {
                    //                 scope.borrow_mut().add_variable(project, AstVariableKind::Local, name, ty);
                    //             }

                    //             _ => {}
                    //         }
                    //     },
                    // );

                    let rhs_context = ExprContext {
                        path: context.path,
                        module: context.module,
                        item: context.item,
                        impl_attributes: context.impl_attributes,
                        item_impl: context.item_impl,
                        fn_attributes: Some(context.fn_attributes),
                        item_fn: Some(context.item_fn),
                        blocks: context.blocks.clone(),
                        statement: context.statement,
                        expr: rhs.as_ref(),
                    };
 
                    self.visit_expr(&rhs_context, parent_scope.clone(), project)?;
                    self.leave_expr(&rhs_context, parent_scope.clone(), project)?;
                }
            }

            let then_block_context = BlockContext {
                path: context.path,
                module: context.module,
                item: context.item,
                impl_attributes: context.impl_attributes,
                item_impl: context.item_impl,
                fn_attributes: context.fn_attributes,
                item_fn: context.item_fn,
                blocks: context.blocks.clone(),
                statement: context.statement,
                expr: Some(context.expr),
                block: &context.if_expr.then_block,
            };

            self.visit_block(&then_block_context, scope.clone(), project)?;
            self.leave_block(&then_block_context, scope.clone(), project)?;    
        }

        if let Some(else_opt) = context.if_expr.else_opt.as_ref() {
            match &else_opt.1 {
                expr::LoopControlFlow::Continue(if_expr) => {
                    let context = IfExprContext {
                        path: context.path,
                        module: context.module,
                        item: context.item,
                        impl_attributes: context.impl_attributes,
                        item_impl: context.item_impl,
                        fn_attributes: context.fn_attributes,
                        item_fn: context.item_fn,
                        blocks: context.blocks.clone(),
                        statement: context.statement,
                        expr: context.expr,
                        if_expr: if_expr.as_ref(),
                    };

                    self.visit_if_expr(&context, scope.clone(), project)?;
                    self.leave_if_expr(&context, scope.clone(), project)?;
                }
                
                expr::LoopControlFlow::Break(else_block) => {
                    let else_block_context = BlockContext {
                        path: context.path,
                        module: context.module,
                        item: context.item,
                        impl_attributes: context.impl_attributes,
                        item_impl: context.item_impl,
                        fn_attributes: context.fn_attributes,
                        item_fn: context.item_fn,
                        blocks: context.blocks.clone(),
                        statement: context.statement,
                        expr: Some(context.expr),
                        block: else_block,
                    };
            
                    self.visit_block(&else_block_context, scope.clone(), project)?;
                    self.leave_block(&else_block_context, scope.clone(), project)?;  
                }
            }
        }
        
        Ok(())
    }

    fn leave_if_expr(&mut self, context: &IfExprContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_if_expr(context, scope.clone(), project)?;
        }
        
        for hook in self.leave_if_expr_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn visit_match_expr(&mut self, context: &MatchExprContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_match_expr(context, scope.clone(), project)?;
        }

        for hook in self.visit_match_expr_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        let value_context = ExprContext {
            path: context.path,
            module: context.module,
            item: context.item,
            impl_attributes: context.impl_attributes,
            item_impl: context.item_impl,
            fn_attributes: Some(context.fn_attributes),
            item_fn: Some(context.item_fn),
            blocks: context.blocks.clone(),
            statement: context.statement,
            expr: context.value,
        };

        self.visit_expr(&value_context, scope.clone(), project)?;
        self.leave_expr(&value_context, scope.clone(), project)?;

        for branch in context.branches.inner.iter() {
            let context = MatchBranchContext {
                path: context.path,
                module: context.module,
                item: context.item,
                impl_attributes: context.impl_attributes,
                item_impl: context.item_impl,
                fn_attributes: context.fn_attributes,
                item_fn: context.item_fn,
                blocks: context.blocks.clone(),
                statement: context.statement,
                expr: context.expr,
                value: context.value,
                branch,
            };

            self.visit_match_branch(&context, scope.clone(), project)?;
            self.leave_match_branch(&context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn leave_match_expr(&mut self, context: &MatchExprContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_match_expr(context, scope.clone(), project)?;
        }
        
        for hook in self.leave_match_expr_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn visit_match_branch(&mut self, context: &MatchBranchContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_match_branch(context, scope.clone(), project)?;
        }

        for hook in self.visit_match_branch_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        //
        // NOTE: `context.branch.pattern` pattern can be handled by overriding `visit_match_branch`
        //

        match &context.branch.kind {
            MatchBranchKind::Block { block, .. } => {
                let context = BlockContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    expr: Some(context.expr),
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    block,
                };

                self.visit_block(&context, scope.clone(), project)?;
                self.leave_block(&context, scope.clone(), project)?;
            }

            MatchBranchKind::Expr { expr, .. } => {
                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: Some(context.fn_attributes),
                    item_fn: Some(context.item_fn),
                    blocks: context.blocks.clone(),
                    statement: context.statement,
                    expr,
                };

                self.visit_expr(&context, scope.clone(), project)?;
                self.leave_expr(&context, scope.clone(), project)?;
            }
        }
        
        Ok(())
    }

    fn leave_match_branch(&mut self, context: &MatchBranchContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_match_branch(context, scope.clone(), project)?;
        }
        
        for hook in self.leave_match_branch_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn visit_while_expr(&mut self, context: &WhileExprContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_while_expr(context, scope.clone(), project)?;
        }

        for hook in self.visit_while_expr_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        let condition_context = ExprContext {
            path: context.path,
            module: context.module,
            item: context.item,
            impl_attributes: context.impl_attributes,
            item_impl: context.item_impl,
            fn_attributes: Some(context.fn_attributes),
            item_fn: Some(context.item_fn),
            blocks: context.blocks.clone(),
            statement: context.statement,
            expr: context.condition,
        };

        self.visit_expr(&condition_context, scope.clone(), project)?;
        self.leave_expr(&condition_context, scope.clone(), project)?;

        let body_context = BlockContext {
            path: context.path,
            module: context.module,
            item: context.item,
            impl_attributes: context.impl_attributes,
            item_impl: context.item_impl,
            fn_attributes: context.fn_attributes,
            item_fn: context.item_fn,
            blocks: context.blocks.clone(),
            statement: context.statement,
            expr: Some(context.expr),
            block: context.body,
        };

        self.visit_block(&body_context, scope.clone(), project)?;
        self.leave_block(&body_context, scope.clone(), project)?;

        Ok(())
    }

    fn leave_while_expr(&mut self, context: &WhileExprContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_while_expr(context, scope.clone(), project)?;
        }
        
        for hook in self.leave_while_expr_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn visit_for_expr(&mut self, context: &ForExprContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_for_expr(context, scope.clone(), project)?;
        }

        for hook in self.visit_for_expr_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        let iterator_context = ExprContext {
            path: context.path,
            module: context.module,
            item: context.item,
            impl_attributes: context.impl_attributes,
            item_impl: context.item_impl,
            fn_attributes: Some(context.fn_attributes),
            item_fn: Some(context.item_fn),
            blocks: context.blocks.clone(),
            statement: context.statement,
            expr: context.iterator,
        };

        self.visit_expr(&iterator_context, scope.clone(), project)?;
        self.leave_expr(&iterator_context, scope.clone(), project)?;

        let body_context = BlockContext {
            path: context.path,
            module: context.module,
            item: context.item,
            impl_attributes: context.impl_attributes,
            item_impl: context.item_impl,
            fn_attributes: context.fn_attributes,
            item_fn: context.item_fn,
            blocks: context.blocks.clone(),
            statement: context.statement,
            expr: Some(context.expr),
            block: context.body,
        };

        self.visit_block(&body_context, scope.clone(), project)?;
        self.leave_block(&body_context, scope.clone(), project)?;

        Ok(())
    }

    fn leave_for_expr(&mut self, context: &ForExprContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_for_expr(context, scope.clone(), project)?;
        }
        
        for hook in self.leave_for_expr_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn visit_trait(&mut self, context: &TraitContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        // scope.borrow_mut().add_trait(project, context.item_trait);
        
        let scope = Rc::new(RefCell::new(AstScope::new(Some(scope.clone()))));
        
        // if let Some(generics) = context.item_trait.generics.as_ref() {
        //     scope.borrow_mut().add_generic_params(project, generics, context.item_trait.where_clause_opt.as_ref());
        // }
        
        for visitor in self.visitors.iter_mut() {
            visitor.visit_trait(context, scope.clone(), project)?;
        }

        for hook in self.visit_trait_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn leave_trait(&mut self, context: &TraitContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_trait(context, scope.clone(), project)?;
        }
        
        for hook in self.leave_trait_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn visit_impl(&mut self, context: &ImplContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        // scope.borrow_mut().add_impl(project, context.item_impl);

        let scope = Rc::new(RefCell::new(AstScope::new(Some(scope.clone()))));
        
        // if let Some(generics) = context.item_impl.generic_params_opt.as_ref() {
        //     scope.borrow_mut().add_generic_params(project, generics, context.item_impl.where_clause_opt.as_ref());
        // }
        
        for visitor in self.visitors.iter_mut() {
            visitor.visit_impl(context, scope.clone(), project)?;
        }

        for hook in self.visit_impl_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        for item in context.item_impl.contents.inner.iter() {
            match &item.value {
                sway_ast::ItemImplItem::Fn(item_fn) => {
                    let context = FnContext {
                        path: context.path,
                        module: context.module,
                        item: &ItemKind::Fn(item_fn.clone()),
                        impl_attributes: Some(context.attributes),
                        item_impl: Some(context.item_impl),
                        fn_attributes: item.attributes.as_slice(),
                        item_fn,
                    };
                    
                    self.visit_fn(&context, scope.clone(), project)?;
                    self.leave_fn(&context, scope.clone(), project)?;
                }

                sway_ast::ItemImplItem::Const(item_const) => {
                    let context = ConstContext {
                        path: context.path,
                        module: context.module,
                        item: &ItemKind::Const(item_const.clone()),
                        impl_attributes: Some(context.attributes),
                        item_impl: Some(context.item_impl),
                        fn_attributes: None,
                        item_fn: None,
                        const_attributes: item.attributes.as_slice(),
                        item_const,
                        blocks: context.blocks.clone(),
                        statement: context.statement,
                    };
                    
                    self.visit_const(&context, scope.clone(), project)?;
                    self.leave_const(&context, scope.clone(), project)?;
                }

                sway_ast::ItemImplItem::Type(item_type) => {
                    let context = TraitTypeContext {
                        path: context.path,
                        module: context.module,
                        item: context.item,
                        attributes: context.attributes,
                        item_type,
                    };

                    self.visit_trait_type(&context, scope.clone(), project)?;
                    self.leave_trait_type(&context, scope.clone(), project)?;
                }
            }
        }

        Ok(())
    }

    fn leave_impl(&mut self, context: &ImplContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_impl(context, scope.clone(), project)?;
        }

        for hook in self.leave_impl_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn visit_abi(&mut self, context: &AbiContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        // scope.borrow_mut().add_abi(project, context.item_abi);

        for visitor in self.visitors.iter_mut() {
            visitor.visit_abi(context, scope.clone(), project)?;
        }
        
        for hook in self.visit_abi_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn leave_abi(&mut self, context: &AbiContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_abi(context, scope.clone(), project)?;
        }
        
        for hook in self.leave_abi_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn visit_const(&mut self, context: &ConstContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        let ty = context.item_const.ty_opt.as_ref()
            .map(|(_, ty)| ty.clone())
            .unwrap_or_else(|| scope.borrow().get_expr_ty(context.item_const.expr_opt.as_ref().unwrap(), project));
        
        // scope.borrow_mut().add_variable(
        //     project,
        //     AstVariableKind::Constant,
        //     &context.item_const.name,
        //     &ty,
        // );

        for visitor in self.visitors.iter_mut() {
            visitor.visit_const(context, scope.clone(), project)?;
        }
        
        for hook in self.visit_const_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        if let Some(expr) = context.item_const.expr_opt.as_ref() {
            let context = ExprContext {
                path: context.path,
                module: context.module,
                item: context.item,
                impl_attributes: context.impl_attributes,
                item_impl: context.item_impl,
                fn_attributes: context.fn_attributes,
                item_fn: context.item_fn,
                blocks: vec![],
                statement: context.statement,
                expr,
            };

            self.visit_expr(&context, scope.clone(), project)?;
            self.leave_expr(&context, scope.clone(), project)?;
        }
        
        Ok(())
    }

    fn leave_const(&mut self, context: &ConstContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_const(context, scope.clone(), project)?;
        }
        
        for hook in self.leave_const_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn visit_storage(&mut self, context: &StorageContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_storage(context, scope.clone(), project)?;
        }

        for hook in self.visit_storage_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        for field in &context.item_storage.entries.inner {
            let context = StorageFieldContext {
                path: context.path,
                module: context.module,
                item: context.item,
                storage_attributes: context.attributes,
                item_storage: context.item_storage,
                field_attributes: field.attributes.as_slice(),
                field: &field.value.field.as_ref().unwrap(),
            };

            self.visit_storage_field(&context, scope.clone(), project)?;
            self.leave_storage_field(&context, scope.clone(), project)?;
        }
        
        Ok(())
    }

    fn leave_storage(&mut self, context: &StorageContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_storage(context, scope.clone(), project)?;
        }
        
        for hook in self.leave_storage_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn visit_storage_field(&mut self, context: &StorageFieldContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        // scope.borrow_mut().add_variable(
        //     project,
        //     AstVariableKind::Storage,
        //     &context.field.name,
        //     &context.field.ty,
        // );

        for visitor in self.visitors.iter_mut() {
            visitor.visit_storage_field(context, scope.clone(), project)?;
        }

        for hook in self.visit_storage_field_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        let context = ExprContext {
            path: context.path,
            module: context.module,
            item: context.item,
            impl_attributes: None,
            item_impl: None,
            fn_attributes: None,
            item_fn: None,
            blocks: vec![],
            statement: None,
            expr: &context.field.initializer,
        };

        self.visit_expr(&context, scope.clone(), project)?;
        self.leave_expr(&context, scope.clone(), project)?;
        
        Ok(())
    }

    fn leave_storage_field(&mut self, context: &StorageFieldContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_storage_field(context, scope.clone(), project)?;
        }
        
        for hook in self.leave_storage_field_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        Ok(())
    }
    
    fn visit_configurable(&mut self, context: &ConfigurableContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_configurable(context, scope.clone(), project)?;
        }

        for hook in self.visit_configurable_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        for field in &context.item_configurable.fields.inner {
            let context = ConfigurableFieldContext {
                path: context.path,
                module: context.module,
                item: context.item,
                configurable_attributes: context.attributes,
                item_configurable: context.item_configurable,
                field_attributes: field.attributes.as_slice(),
                field: &field.value,
            };

            self.visit_configurable_field(&context, scope.clone(), project)?;
            self.leave_configurable_field(&context, scope.clone(), project)?;
        }
        
        Ok(())
    }

    fn leave_configurable(&mut self, context: &ConfigurableContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_configurable(context, scope.clone(), project)?;
        }
        
        for hook in self.leave_configurable_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn visit_configurable_field(&mut self, context: &ConfigurableFieldContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        // scope.borrow_mut().add_variable(
        //     project,
        //     AstVariableKind::Configurable,
        //     &context.field.name,
        //     &context.field.ty,
        // );

        for visitor in self.visitors.iter_mut() {
            visitor.visit_configurable_field(context, scope.clone(), project)?;
        }

        for hook in self.visit_configurable_field_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        let context = ExprContext {
            path: context.path,
            module: context.module,
            item: context.item,
            impl_attributes: None,
            item_impl: None,
            fn_attributes: None,
            item_fn: None,
            blocks: vec![],
            statement: None,
            expr: &context.field.initializer,
        };

        self.visit_expr(&context, scope.clone(), project)?;
        self.leave_expr(&context, scope.clone(), project)?;
        
        Ok(())
    }

    fn leave_configurable_field(&mut self, context: &ConfigurableFieldContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_configurable_field(context, scope.clone(), project)?;
        }
        
        for hook in self.leave_configurable_field_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn visit_type_alias(&mut self, context: &TypeAliasContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        // scope.borrow_mut().add_type_alias(project, context.item_type_alias);

        for visitor in self.visitors.iter_mut() {
            visitor.visit_type_alias(context, scope.clone(), project)?;
        }
        
        for hook in self.visit_type_alias_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn leave_type_alias(&mut self, context: &TypeAliasContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_type_alias(context, scope.clone(), project)?;
        }
        
        for hook in self.leave_type_alias_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn visit_trait_type(&mut self, context: &TraitTypeContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_trait_type(context, scope.clone(), project)?;
        }
        
        for hook in self.visit_trait_type_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        Ok(())
    }

    fn leave_trait_type(&mut self, context: &TraitTypeContext, scope: Rc<RefCell<AstScope>>, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_trait_type(context, scope.clone(), project)?;
        }
        
        for hook in self.leave_trait_type_hooks.iter_mut() {
            hook(context, scope.clone(), project)?;
        }

        Ok(())
    }
}
