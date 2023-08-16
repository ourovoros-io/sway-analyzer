use crate::{error::Error, project::Project, utils};
use std::path::Path;
use sway_ast::{*, attribute::Annotated, expr::asm::AsmFinalExpr};
use sway_types::{Span, Spanned};

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

#[allow(unused_variables)]
pub trait AstVisitor {
    fn visit_module(&mut self, context: &ModuleContext, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_module(&mut self, context: &ModuleContext, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_module_item(&mut self, context: &ItemContext, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_module_item(&mut self, context: &ItemContext, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_submodule(&mut self, context: &SubmoduleContext, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_submodule(&mut self, context: &SubmoduleContext, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_use(&mut self, context: &UseContext, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_use(&mut self, context: &UseContext, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_struct(&mut self, context: &StructContext, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_struct(&mut self, context: &StructContext, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_struct_field(&mut self, context: &StructFieldContext, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_struct_field(&mut self, context: &StructFieldContext, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_enum(&mut self, context: &EnumContext, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_enum(&mut self, context: &EnumContext, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_enum_field(&mut self, context: &EnumFieldContext, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_enum_field(&mut self, context: &EnumFieldContext, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_fn(&mut self, context: &FnContext, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_fn(&mut self, context: &FnContext, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_statement(&mut self, context: &StatementContext, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_statement(&mut self, context: &StatementContext, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_statement_let(&mut self, context: &StatementLetContext, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_statement_let(&mut self, context: &StatementLetContext, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_expr(&mut self, context: &ExprContext, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_expr(&mut self, context: &ExprContext, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_block(&mut self, context: &BlockContext, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_block(&mut self, context: &BlockContext, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_asm_block(&mut self, context: &AsmBlockContext, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_asm_block(&mut self, context: &AsmBlockContext, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_asm_instruction(&mut self, context: &AsmInstructionContext, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_asm_instruction(&mut self, context: &AsmInstructionContext, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_asm_final_expr(&mut self, context: &AsmFinalExprContext, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_asm_final_expr(&mut self, context: &AsmFinalExprContext, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_if_expr(&mut self, context: &IfExprContext, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_if_expr(&mut self, context: &IfExprContext, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_match_expr(&mut self, context: &MatchExprContext, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_match_expr(&mut self, context: &MatchExprContext, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_match_branch(&mut self, context: &MatchBranchContext, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_match_branch(&mut self, context: &MatchBranchContext, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_while_expr(&mut self, context: &WhileExprContext, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_while_expr(&mut self, context: &WhileExprContext, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_trait(&mut self, context: &TraitContext, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_trait(&mut self, context: &TraitContext, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_impl(&mut self, context: &ImplContext, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_impl(&mut self, context: &ImplContext, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_abi(&mut self, context: &AbiContext, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_abi(&mut self, context: &AbiContext, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_const(&mut self, context: &ConstContext, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_const(&mut self, context: &ConstContext, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_storage(&mut self, context: &StorageContext, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_storage(&mut self, context: &StorageContext, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_storage_field(&mut self, context: &StorageFieldContext, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_storage_field(&mut self, context: &StorageFieldContext, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_configurable(&mut self, context: &ConfigurableContext, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_configurable(&mut self, context: &ConfigurableContext, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_configurable_field(&mut self, context: &ConfigurableFieldContext, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_configurable_field(&mut self, context: &ConfigurableFieldContext, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_type_alias(&mut self, context: &TypeAliasContext, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_type_alias(&mut self, context: &TypeAliasContext, project: &mut Project) -> Result<(), Error> { Ok(()) }
}

#[derive(Default)]
pub struct AstVisitorRecursive {
    pub visitors: Vec<Box<dyn AstVisitor>>,
}

impl AstVisitor for AstVisitorRecursive {
    fn visit_module(&mut self, context: &ModuleContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_module(context, project)?;
        }

        for item in context.module.items.iter() {
            let context = ItemContext {
                path: context.path,
                module: context.module,
                attributes: item.attribute_list.as_slice(),
                item: &item.value,
                impl_attributes: None,
                item_impl: None,
                fn_attributes: None,
                item_fn: None,
                blocks: vec![],
                statement: None,
            };
            
            self.visit_module_item(&context, project)?;
            self.leave_module_item(&context, project)?;
        }

        Ok(())
    }

    fn leave_module(&mut self, context: &ModuleContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_module(context, project)?;
        }

        Ok(())
    }

    fn visit_module_item(&mut self, context: &ItemContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_module_item(context, project)?;
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
                
                self.visit_submodule(&context, project)?;
                self.leave_submodule(&context, project)?;
            }

            ItemKind::Use(item_use) => {
                let context = UseContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    attributes: context.attributes,
                    item_use,
                };
                
                self.visit_use(&context, project)?;
                self.leave_use(&context, project)?;
            }

            ItemKind::Struct(item_struct) => {
                let context = StructContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    struct_attributes: context.attributes,
                    item_struct,
                };
                
                self.visit_struct(&context, project)?;
                self.leave_struct(&context, project)?;
            }

            ItemKind::Enum(item_enum) => {
                let context = EnumContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    attributes: context.attributes,
                    item_enum,
                };
                
                self.visit_enum(&context, project)?;
                self.leave_enum(&context, project)?;
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
                
                self.visit_fn(&context, project)?;
                self.leave_fn(&context, project)?;
            }

            ItemKind::Trait(item_trait) => {
                let context = TraitContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    attributes: context.attributes,
                    item_trait,
                };
                
                self.visit_trait(&context, project)?;
                self.leave_trait(&context, project)?;
            }

            ItemKind::Impl(item_impl) => {
                let context = ImplContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    attributes: context.attributes,
                    item_impl,
                    blocks: context.blocks.clone(),
                    statement: context.statement.clone(),
                };
                
                self.visit_impl(&context, project)?;
                self.leave_impl(&context, project)?;
            }

            ItemKind::Abi(item_abi) => {
                let context = AbiContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    attributes: context.attributes,
                    item_abi,
                };
                
                self.visit_abi(&context, project)?;
                self.leave_abi(&context, project)?;
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
                
                self.visit_const(&context, project)?;
                self.leave_const(&context, project)?;
            }

            ItemKind::Storage(item_storage) => {
                let context = StorageContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    attributes: context.attributes,
                    item_storage,
                };
                
                self.visit_storage(&context, project)?;
                self.leave_storage(&context, project)?;
            }

            ItemKind::Configurable(item_configurable) => {
                let context = ConfigurableContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    attributes: context.attributes,
                    item_configurable,
                };
                
                self.visit_configurable(&context, project)?;
                self.leave_configurable(&context, project)?;
            }

            ItemKind::TypeAlias(item_type_alias) => {
                let context = TypeAliasContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    attributes: context.attributes,
                    item_type_alias,
                };
                
                self.visit_type_alias(&context, project)?;
                self.leave_type_alias(&context, project)?;
            }
        }

        Ok(())
    }

    fn leave_module_item(&mut self, context: &ItemContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_module_item(context, project)?;
        }

        Ok(())
    }

    fn visit_submodule(&mut self, context: &SubmoduleContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_submodule(context, project)?;
        }

        Ok(())
    }

    fn leave_submodule(&mut self, context: &SubmoduleContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_submodule(context, project)?;
        }
        
        Ok(())
    }

    fn visit_use(&mut self, context: &UseContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_use(context, project)?;
        }
        
        Ok(())
    }

    fn leave_use(&mut self, context: &UseContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_use(context, project)?;
        }
        
        Ok(())
    }

    fn visit_struct(&mut self, context: &StructContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_struct(context, project)?;
        }
        
        for field in utils::fold_punctuated(&context.item_struct.fields.inner) {
            let context = StructFieldContext {
                path: context.path,
                module: context.module,
                item: context.item,
                struct_attributes: context.struct_attributes,
                item_struct: context.item_struct,
                field_attributes: field.attribute_list.as_slice(),
                field: &field.value,
            };

            self.visit_struct_field(&context, project)?;
            self.leave_struct_field(&context, project)?;
        }

        Ok(())
    }

    fn leave_struct(&mut self, context: &StructContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_struct(context, project)?;
        }
        
        Ok(())
    }

    fn visit_struct_field(&mut self, context: &StructFieldContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_struct_field(context, project)?;
        }

        Ok(())
    }

    fn leave_struct_field(&mut self, context: &StructFieldContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_struct_field(context, project)?;
        }

        Ok(())
    }

    fn visit_enum(&mut self, context: &EnumContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_enum(context, project)?;
        }
        
        for field in utils::fold_punctuated(&context.item_enum.fields.inner) {
            let context = EnumFieldContext {
                path: context.path,
                module: context.module,
                item: context.item,
                enum_attributes: context.attributes,
                item_enum: context.item_enum,
                field_attributes: field.attribute_list.as_slice(),
                field: &field.value,
            };

            self.visit_enum_field(&context, project)?;
            self.leave_enum_field(&context, project)?;
        }

        Ok(())
    }

    fn leave_enum(&mut self, context: &EnumContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_enum(context, project)?;
        }
        
        Ok(())
    }

    fn visit_enum_field(&mut self, context: &EnumFieldContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_enum_field(context, project)?;
        }
        
        Ok(())
    }

    fn leave_enum_field(&mut self, context: &EnumFieldContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_enum_field(context, project)?;
        }
        
        Ok(())
    }

    fn visit_fn(&mut self, context: &FnContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_fn(context, project)?;
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

        self.visit_block(&body_context, project)?;
        self.leave_block(&body_context, project)?;
        
        Ok(())
    }

    fn leave_fn(&mut self, context: &FnContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_fn(context, project)?;
        }
        
        Ok(())
    }

    fn visit_statement(&mut self, context: &StatementContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_statement(context, project)?;
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

                self.visit_statement_let(&context, project)?;
                self.leave_statement_let(&context, project)?;
            }

            Statement::Item(item) => {
                let context = ItemContext {
                    path: context.path,
                    module: context.module,
                    attributes: item.attribute_list.as_slice(),
                    item: &item.value,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: Some(context.fn_attributes),
                    item_fn: Some(context.item_fn),
                    blocks: context.blocks.clone(),
                    statement: Some(context.statement),
                };

                self.visit_module_item(&context, project)?;
                self.leave_module_item(&context, project)?;
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

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;
            }
        }

        Ok(())
    }

    fn leave_statement(&mut self, context: &StatementContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_statement(context, project)?;
        }
        
        Ok(())
    }

    fn visit_statement_let(&mut self, context: &StatementLetContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_statement_let(context, project)?;
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

        self.visit_expr(&context, project)?;
        self.leave_expr(&context, project)?;

        Ok(())
    }

    fn leave_statement_let(&mut self, context: &StatementLetContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_statement_let(context, project)?;
        }
        
        Ok(())
    }

    fn visit_expr(&mut self, context: &ExprContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_expr(context, project)?;
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
                    statement: context.statement.clone(),
                    expr: args.inner.address.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;
            }

            Expr::Struct { fields, .. } => {
                for field in utils::fold_punctuated(&fields.inner) {
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
                            statement: context.statement.clone(),
                            expr: field.1.as_ref(),
                        };
        
                        self.visit_expr(&context, project)?;
                        self.leave_expr(&context, project)?;
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
                            statement: context.statement.clone(),
                            expr: head.as_ref(),
                        };
        
                        self.visit_expr(&context, project)?;
                        self.leave_expr(&context, project)?;

                        for expr in utils::fold_punctuated(tail) {
                            let context = ExprContext {
                                path: context.path,
                                module: context.module,
                                item: context.item,
                                impl_attributes: context.impl_attributes,
                                item_impl: context.item_impl,
                                fn_attributes: context.fn_attributes,
                                item_fn: context.item_fn,
                                blocks: context.blocks.clone(),
                                statement: context.statement.clone(),
                                expr,
                            };
            
                            self.visit_expr(&context, project)?;
                            self.leave_expr(&context, project)?;
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
                    statement: context.statement.clone(),
                    expr: parens.inner.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;
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
                    statement: context.statement.clone(),
                    block,
                };

                self.visit_block(&context, project)?;
                self.leave_block(&context, project)?;
            }
            
            Expr::Array(array) => {
                match &array.inner {
                    ExprArrayDescriptor::Sequence(sequence) => {
                        for expr in utils::fold_punctuated(sequence) {
                            let context = ExprContext {
                                path: context.path,
                                module: context.module,
                                item: context.item,
                                impl_attributes: context.impl_attributes,
                                item_impl: context.item_impl,
                                fn_attributes: context.fn_attributes,
                                item_fn: context.item_fn,
                                blocks: context.blocks.clone(),
                                statement: context.statement.clone(),
                                expr,
                            };
            
                            self.visit_expr(&context, project)?;
                            self.leave_expr(&context, project)?;
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
                            statement: context.statement.clone(),
                            expr: value.as_ref(),
                        };
        
                        self.visit_expr(&context, project)?;
                        self.leave_expr(&context, project)?;

                        let context = ExprContext {
                            path: context.path,
                            module: context.module,
                            item: context.item,
                            impl_attributes: context.impl_attributes,
                            item_impl: context.item_impl,
                            fn_attributes: context.fn_attributes,
                            item_fn: context.item_fn,
                            blocks: context.blocks.clone(),
                            statement: context.statement.clone(),
                            expr: length.as_ref(),
                        };

                        self.visit_expr(&context, project)?;
                        self.leave_expr(&context, project)?;
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
                    statement: context.statement.clone(),
                    expr: context.expr,
                    asm,
                };

                self.visit_asm_block(&context, project)?;
                self.leave_asm_block(&context, project)?;
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
                        statement: context.statement.clone(),
                        expr: expr.as_ref(),
                    };
    
                    self.visit_expr(&context, project)?;
                    self.leave_expr(&context, project)?;
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
                    statement: context.statement.clone(),
                    expr: context.expr,
                    if_expr,
                };

                self.visit_if_expr(&context, project)?;
                self.leave_if_expr(&context, project)?;
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
                    statement: context.statement.clone(),
                    expr: context.expr,
                    value: value.as_ref(),
                    branches,
                };

                self.visit_match_expr(&context, project)?;
                self.leave_match_expr(&context, project)?;
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
                    statement: context.statement.clone(),
                    expr: context.expr,
                    condition: condition.as_ref(),
                    body: block,
                };

                self.visit_while_expr(&context, project)?;
                self.leave_while_expr(&context, project)?;
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
                    statement: context.statement.clone(),
                    expr: func.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;

                for arg in utils::fold_punctuated(&args.inner) {
                    let context = ExprContext {
                        path: context.path,
                        module: context.module,
                        item: context.item,
                        impl_attributes: context.impl_attributes,
                        item_impl: context.item_impl,
                        fn_attributes: context.fn_attributes,
                        item_fn: context.item_fn,
                        blocks: context.blocks.clone(),
                        statement: context.statement.clone(),
                        expr: arg,
                    };
    
                    self.visit_expr(&context, project)?;
                    self.leave_expr(&context, project)?;    
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
                    statement: context.statement.clone(),
                    expr: target.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;

                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement.clone(),
                    expr: arg.inner.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;
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
                    statement: context.statement.clone(),
                    expr: target.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;

                if let Some(opts) = contract_args_opt.as_ref() {
                    for expr in utils::fold_punctuated(&opts.inner) {
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
                                statement: context.statement.clone(),
                                expr: expr.1.as_ref(),
                            };
            
                            self.visit_expr(&context, project)?;
                            self.leave_expr(&context, project)?;
                        }
                    }
                }
            
                for arg in utils::fold_punctuated(&args.inner) {
                    let context = ExprContext {
                        path: context.path,
                        module: context.module,
                        item: context.item,
                        impl_attributes: context.impl_attributes,
                        item_impl: context.item_impl,
                        fn_attributes: context.fn_attributes,
                        item_fn: context.item_fn,
                        blocks: context.blocks.clone(),
                        statement: context.statement.clone(),
                        expr: arg,
                    };
    
                    self.visit_expr(&context, project)?;
                    self.leave_expr(&context, project)?;
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
                    statement: context.statement.clone(),
                    expr: target.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;
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
                    statement: context.statement.clone(),
                    expr: target.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;
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
                    statement: context.statement.clone(),
                    expr: expr.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;
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
                    statement: context.statement.clone(),
                    expr: expr.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;
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
                    statement: context.statement.clone(),
                    expr: expr.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;
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
                    statement: context.statement.clone(),
                    expr: lhs.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;

                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement.clone(),
                    expr: rhs.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;
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
                    statement: context.statement.clone(),
                    expr: lhs.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;

                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement.clone(),
                    expr: rhs.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;
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
                    statement: context.statement.clone(),
                    expr: lhs.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;

                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement.clone(),
                    expr: rhs.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;
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
                    statement: context.statement.clone(),
                    expr: lhs.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;

                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement.clone(),
                    expr: rhs.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;
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
                    statement: context.statement.clone(),
                    expr: lhs.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;

                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement.clone(),
                    expr: rhs.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;
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
                    statement: context.statement.clone(),
                    expr: lhs.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;

                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement.clone(),
                    expr: rhs.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;
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
                    statement: context.statement.clone(),
                    expr: lhs.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;

                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement.clone(),
                    expr: rhs.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;
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
                    statement: context.statement.clone(),
                    expr: lhs.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;

                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement.clone(),
                    expr: rhs.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;
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
                    statement: context.statement.clone(),
                    expr: lhs.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;

                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement.clone(),
                    expr: rhs.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;
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
                    statement: context.statement.clone(),
                    expr: lhs.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;

                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement.clone(),
                    expr: rhs.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;
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
                    statement: context.statement.clone(),
                    expr: lhs.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;

                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement.clone(),
                    expr: rhs.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;
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
                    statement: context.statement.clone(),
                    expr: lhs.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;

                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement.clone(),
                    expr: rhs.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;
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
                    statement: context.statement.clone(),
                    expr: lhs.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;

                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement.clone(),
                    expr: rhs.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;
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
                    statement: context.statement.clone(),
                    expr: lhs.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;

                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement.clone(),
                    expr: rhs.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;
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
                    statement: context.statement.clone(),
                    expr: lhs.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;

                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement.clone(),
                    expr: rhs.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;
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
                    statement: context.statement.clone(),
                    expr: lhs.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;

                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement.clone(),
                    expr: rhs.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;
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
                    statement: context.statement.clone(),
                    expr: lhs.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;

                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement.clone(),
                    expr: rhs.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;
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
                    statement: context.statement.clone(),
                    expr: lhs.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;

                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement.clone(),
                    expr: rhs.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;
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
                    statement: context.statement.clone(),
                    expr: lhs.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;

                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
                    blocks: context.blocks.clone(),
                    statement: context.statement.clone(),
                    expr: rhs.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;
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
                    statement: context.statement.clone(),
                    expr: expr.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;
            }

            Expr::Break { .. } => {}
            Expr::Continue { .. } => {}
        }
        
        Ok(())
    }

    fn leave_expr(&mut self, context: &ExprContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_expr(context, project)?;
        }
        
        Ok(())
    }

    fn visit_block(&mut self, context: &BlockContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_block(context, project)?;
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

            self.visit_statement(&context, project)?;
            self.leave_statement(&context, project)?;
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
                statement: context.statement.clone(),
                expr: expr.as_ref(),
            };

            self.visit_expr(&context, project)?;
            self.leave_expr(&context, project)?;
        }
        
        Ok(())
    }

    fn leave_block(&mut self, context: &BlockContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_block(context, project)?;
        }
        
        Ok(())
    }

    fn visit_asm_block(&mut self, context: &AsmBlockContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_asm_block(context, project)?;
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
                statement: context.statement.clone(),
                expr: context.expr,
                asm: context.asm,
                instruction: &instruction.0,
            };

            self.visit_asm_instruction(&context, project)?;
            self.leave_asm_instruction(&context, project)?;
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
                statement: context.statement.clone(),
                expr: context.expr,
                asm: context.asm,
                final_expr,
            };

            self.visit_asm_final_expr(&context, project)?;
            self.leave_asm_final_expr(&context, project)?;
        }

        Ok(())
    }

    fn leave_asm_block(&mut self, context: &AsmBlockContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_asm_block(context, project)?;
        }
        
        Ok(())
    }

    fn visit_asm_instruction(&mut self, context: &AsmInstructionContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_asm_instruction(context, project)?;
        }
        
        Ok(())
    }

    fn leave_asm_instruction(&mut self, context: &AsmInstructionContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_asm_instruction(context, project)?;
        }
        
        Ok(())
    }

    fn visit_asm_final_expr(&mut self, context: &AsmFinalExprContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_asm_final_expr(context, project)?;
        }
        
        Ok(())
    }

    fn leave_asm_final_expr(&mut self, context: &AsmFinalExprContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_asm_final_expr(context, project)?;
        }
        
        Ok(())
    }

    fn visit_if_expr(&mut self, context: &IfExprContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_if_expr(context, project)?;
        }

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
                    statement: context.statement.clone(),
                    expr: expr.as_ref(),
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;
            }
            
            IfCondition::Let { rhs, .. } => {
                //
                // NOTE: `lhs` pattern can be handled by overriding `visit_if_expr`
                //

                let rhs_context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: Some(context.fn_attributes),
                    item_fn: Some(context.item_fn),
                    blocks: context.blocks.clone(),
                    statement: context.statement.clone(),
                    expr: rhs.as_ref(),
                };

                self.visit_expr(&rhs_context, project)?;
                self.leave_expr(&rhs_context, project)?;
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
            statement: context.statement.clone(),
            expr: Some(context.expr),
            block: &context.if_expr.then_block,
        };

        self.visit_block(&then_block_context, project)?;
        self.leave_block(&then_block_context, project)?;

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
                        statement: context.statement.clone(),
                        expr: context.expr,
                        if_expr: if_expr.as_ref(),
                    };

                    self.visit_if_expr(&context, project)?;
                    self.leave_if_expr(&context, project)?;
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
                        statement: context.statement.clone(),
                        expr: Some(context.expr),
                        block: else_block,
                    };
            
                    self.visit_block(&else_block_context, project)?;
                    self.leave_block(&else_block_context, project)?;  
                }
            }
        }
        
        Ok(())
    }

    fn leave_if_expr(&mut self, context: &IfExprContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_if_expr(context, project)?;
        }
        
        Ok(())
    }

    fn visit_match_expr(&mut self, context: &MatchExprContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_match_expr(context, project)?;
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
            statement: context.statement.clone(),
            expr: context.value,
        };

        self.visit_expr(&value_context, project)?;
        self.leave_expr(&value_context, project)?;

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
                statement: context.statement.clone(),
                expr: context.expr,
                value: context.value,
                branch,
            };

            self.visit_match_branch(&context, project)?;
            self.leave_match_branch(&context, project)?;
        }

        Ok(())
    }

    fn leave_match_expr(&mut self, context: &MatchExprContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_match_expr(context, project)?;
        }
        
        Ok(())
    }

    fn visit_match_branch(&mut self, context: &MatchBranchContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_match_branch(context, project)?;
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
                    statement: context.statement.clone(),
                    block,
                };

                self.visit_block(&context, project)?;
                self.leave_block(&context, project)?;
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
                    statement: context.statement.clone(),
                    expr,
                };

                self.visit_expr(&context, project)?;
                self.leave_expr(&context, project)?;
            }
        }
        
        Ok(())
    }

    fn leave_match_branch(&mut self, context: &MatchBranchContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_match_branch(context, project)?;
        }
        
        Ok(())
    }

    fn visit_while_expr(&mut self, context: &WhileExprContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_while_expr(context, project)?;
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
            statement: context.statement.clone(),
            expr: context.condition,
        };

        self.visit_expr(&condition_context, project)?;
        self.leave_expr(&condition_context, project)?;

        let body_context = BlockContext {
            path: context.path,
            module: context.module,
            item: context.item,
            impl_attributes: context.impl_attributes,
            item_impl: context.item_impl,
            fn_attributes: context.fn_attributes,
            item_fn: context.item_fn,
            blocks: context.blocks.clone(),
            statement: context.statement.clone(),
            expr: Some(context.expr),
            block: context.body,
        };

        self.visit_block(&body_context, project)?;
        self.leave_block(&body_context, project)?;

        Ok(())
    }

    fn leave_while_expr(&mut self, context: &WhileExprContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_while_expr(context, project)?;
        }
        
        Ok(())
    }

    fn visit_trait(&mut self, context: &TraitContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_trait(context, project)?;
        }

        Ok(())
    }

    fn leave_trait(&mut self, context: &TraitContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_trait(context, project)?;
        }
        
        Ok(())
    }

    fn visit_impl(&mut self, context: &ImplContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_impl(context, project)?;
        }

        for item in context.item_impl.contents.inner.iter() {
            match &item.value {
                sway_ast::ItemImplItem::Fn(item_fn) => {
                    let context = FnContext {
                        path: context.path,
                        module: context.module,
                        item: context.item,
                        impl_attributes: Some(context.attributes),
                        item_impl: Some(context.item_impl),
                        fn_attributes: item.attribute_list.as_slice(),
                        item_fn,
                    };
                    
                    self.visit_fn(&context, project)?;
                    self.leave_fn(&context, project)?;
                }

                sway_ast::ItemImplItem::Const(item_const) => {
                    let context = ConstContext {
                        path: context.path,
                        module: context.module,
                        item: context.item,
                        impl_attributes: Some(context.attributes),
                        item_impl: Some(context.item_impl),
                        fn_attributes: None,
                        item_fn: None,
                        const_attributes: item.attribute_list.as_slice(),
                        item_const,
                        blocks: context.blocks.clone(),
                        statement: context.statement.clone(),
                    };
                    
                    self.visit_const(&context, project)?;
                    self.leave_const(&context, project)?;
                }
            }
        }

        Ok(())
    }

    fn leave_impl(&mut self, context: &ImplContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_impl(context, project)?;
        }

        Ok(())
    }

    fn visit_abi(&mut self, context: &AbiContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_abi(context, project)?;
        }
        
        Ok(())
    }

    fn leave_abi(&mut self, context: &AbiContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_abi(context, project)?;
        }
        
        Ok(())
    }

    fn visit_const(&mut self, context: &ConstContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_const(context, project)?;
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
                statement: context.statement.clone(),
                expr,
            };

            self.visit_expr(&context, project)?;
            self.leave_expr(&context, project)?;
        }
        
        Ok(())
    }

    fn leave_const(&mut self, context: &ConstContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_const(context, project)?;
        }
        
        Ok(())
    }

    fn visit_storage(&mut self, context: &StorageContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_storage(context, project)?;
        }

        let mut visit_field = |field: &Annotated<StorageField>| -> Result<(), Error> {
            let context = StorageFieldContext {
                path: context.path,
                module: context.module,
                item: context.item,
                storage_attributes: context.attributes,
                item_storage: context.item_storage,
                field_attributes: field.attribute_list.as_slice(),
                field: &field.value,
            };

            self.visit_storage_field(&context, project)?;
            self.leave_storage_field(&context, project)?;

            Ok(())
        };

        for field in utils::fold_punctuated(&context.item_storage.fields.inner) {
            visit_field(field)?;
        }
        
        Ok(())
    }

    fn leave_storage(&mut self, context: &StorageContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_storage(context, project)?;
        }
        
        Ok(())
    }

    fn visit_storage_field(&mut self, context: &StorageFieldContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_storage_field(context, project)?;
        }
        
        Ok(())
    }

    fn leave_storage_field(&mut self, context: &StorageFieldContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_storage_field(context, project)?;
        }
        
        Ok(())
    }
    
    fn visit_configurable(&mut self, context: &ConfigurableContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_configurable(context, project)?;
        }

        let mut visit_field = |field: &Annotated<ConfigurableField>| -> Result<(), Error> {
            let context = ConfigurableFieldContext {
                path: context.path,
                module: context.module,
                item: context.item,
                configurable_attributes: context.attributes,
                item_configurable: context.item_configurable,
                field_attributes: field.attribute_list.as_slice(),
                field: &field.value,
            };

            self.visit_configurable_field(&context, project)?;
            self.leave_configurable_field(&context, project)?;

            Ok(())
        };

        for field in utils::fold_punctuated(&context.item_configurable.fields.inner) {
            visit_field(field)?;
        }
        
        Ok(())
    }

    fn leave_configurable(&mut self, context: &ConfigurableContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_configurable(context, project)?;
        }
        
        Ok(())
    }

    fn visit_configurable_field(&mut self, context: &ConfigurableFieldContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_configurable_field(context, project)?;
        }
        
        Ok(())
    }

    fn leave_configurable_field(&mut self, context: &ConfigurableFieldContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_configurable_field(context, project)?;
        }
        
        Ok(())
    }

    fn visit_type_alias(&mut self, context: &TypeAliasContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_type_alias(context, project)?;
        }
        
        Ok(())
    }

    fn leave_type_alias(&mut self, context: &TypeAliasContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_type_alias(context, project)?;
        }
        
        Ok(())
    }
}
