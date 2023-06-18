use crate::{error::Error, project::Project};
use std::path::Path;
use sway_ast::{*, attribute::Annotated};

#[derive(Clone)]
pub struct ModuleContext<'module> {
    pub path: &'module Path,
    pub module: &'module Module,
}

#[derive(Clone)]
pub struct ModuleItemContext<'module, 'attributes, 'item> {
    pub path: &'module Path,
    pub module: &'module Module,
    pub attributes: &'attributes [AttributeDecl],
    pub item: &'item ItemKind,
}

#[derive(Clone)]
pub struct SubmoduleContext<'module, 'item, 'attributes, 'submodule> {
    pub path: &'module Path,
    pub module: &'module Module,
    pub item: &'item ItemKind,
    pub attributes: &'attributes [AttributeDecl],
    pub submodule: &'submodule Submodule,
}

#[derive(Clone)]
pub struct UseContext<'module, 'item, 'attributes, 'item_use> {
    pub path: &'module Path,
    pub module: &'module Module,
    pub item: &'item ItemKind,
    pub attributes: &'attributes [AttributeDecl],
    pub item_use: &'item_use ItemUse,
}

#[derive(Clone)]
pub struct StructContext<'module, 'item, 'attributes, 'item_struct> {
    pub path: &'module Path,
    pub module: &'module Module,
    pub item: &'item ItemKind,
    pub attributes: &'attributes [AttributeDecl],
    pub item_struct: &'item_struct ItemStruct,
}

#[derive(Clone)]
pub struct StructFieldContext<'module, 'item, 'struct_attributes, 'item_struct, 'field_attributes, 'field> {
    pub path: &'module Path,
    pub module: &'module Module,
    pub item: &'item ItemKind,
    pub struct_attributes: &'struct_attributes [AttributeDecl],
    pub item_struct: &'item_struct ItemStruct,
    pub field_attributes: &'field_attributes [AttributeDecl],
    pub field: &'field TypeField,
}

#[derive(Clone)]
pub struct EnumContext<'module, 'item, 'attributes, 'item_enum> {
    pub path: &'module Path,
    pub module: &'module Module,
    pub item: &'item ItemKind,
    pub attributes: &'attributes [AttributeDecl],
    pub item_enum: &'item_enum ItemEnum,
}

#[derive(Clone)]
pub struct EnumFieldContext<'module, 'item, 'enum_attributes, 'item_enum, 'field_attributes, 'field> {
    pub path: &'module Path,
    pub module: &'module Module,
    pub item: &'item ItemKind,
    pub enum_attributes: &'enum_attributes [AttributeDecl],
    pub item_enum: &'item_enum ItemEnum,
    pub field_attributes: &'field_attributes [AttributeDecl],
    pub field: &'field TypeField,
}

#[derive(Clone)]
pub struct FnContext<'module, 'item, 'impl_attributes, 'item_impl, 'fn_attributes, 'item_fn> {
    pub path: &'module Path,
    pub module: &'module Module,
    pub item: &'item ItemKind,
    pub impl_attributes: Option<&'impl_attributes [AttributeDecl]>,
    pub item_impl: Option<&'item_impl ItemImpl>,
    pub fn_attributes: &'fn_attributes [AttributeDecl],
    pub item_fn: &'item_fn ItemFn,
}

#[derive(Clone)]
pub struct StatementContext<'module, 'item, 'impl_attributes, 'item_impl, 'fn_attributes, 'item_fn, 'statement> {
    pub path: &'module Path,
    pub module: &'module Module,
    pub item: &'item ItemKind,
    pub impl_attributes: Option<&'impl_attributes [AttributeDecl]>,
    pub item_impl: Option<&'item_impl ItemImpl>,
    pub fn_attributes: &'fn_attributes [AttributeDecl],
    pub item_fn: &'item_fn ItemFn,
    pub statement: &'statement Statement,
}

#[derive(Clone)]
pub struct ExprContext<'module, 'item, 'impl_attributes, 'item_impl, 'fn_attributes, 'item_fn, 'expr> {
    pub path: &'module Path,
    pub module: &'module Module,
    pub item: &'item ItemKind,
    pub impl_attributes: Option<&'impl_attributes [AttributeDecl]>,
    pub item_impl: Option<&'item_impl ItemImpl>,
    pub fn_attributes: &'fn_attributes [AttributeDecl],
    pub item_fn: &'item_fn ItemFn,
    pub expr: &'expr Expr,
}

#[derive(Clone)]
pub struct StatementLetContext<'module, 'item, 'impl_attributes, 'item_impl, 'fn_attributes, 'item_fn, 'statement, 'statement_let> {
    pub path: &'module Path,
    pub module: &'module Module,
    pub item: &'item ItemKind,
    pub impl_attributes: Option<&'impl_attributes [AttributeDecl]>,
    pub item_impl: Option<&'item_impl ItemImpl>,
    pub fn_attributes: &'fn_attributes [AttributeDecl],
    pub item_fn: &'item_fn ItemFn,
    pub statement: &'statement Statement,
    pub statement_let: &'statement_let StatementLet,
}

#[derive(Clone)]
pub struct TraitContext<'module, 'item, 'attributes, 'item_trait> {
    pub path: &'module Path,
    pub module: &'module Module,
    pub item: &'item ItemKind,
    pub attributes: &'attributes [AttributeDecl],
    pub item_trait: &'item_trait ItemTrait,
}

#[derive(Clone)]
pub struct ImplContext<'module, 'item, 'attributes, 'item_impl> {
    pub path: &'module Path,
    pub module: &'module Module,
    pub item: &'item ItemKind,
    pub attributes: &'attributes [AttributeDecl],
    pub item_impl: &'item_impl ItemImpl,
}

#[derive(Clone)]
pub struct AbiContext<'module, 'item, 'attributes, 'item_abi> {
    pub path: &'module Path,
    pub module: &'module Module,
    pub item: &'item ItemKind,
    pub attributes: &'attributes [AttributeDecl],
    pub item_abi: &'item_abi ItemAbi,
}

#[derive(Clone)]
pub struct ConstContext<'module, 'item, 'impl_attributes, 'item_impl, 'const_attributes, 'item_const> {
    pub path: &'module Path,
    pub module: &'module Module,
    pub item: &'item ItemKind,
    pub impl_attributes: Option<&'impl_attributes [AttributeDecl]>,
    pub item_impl: Option<&'item_impl ItemImpl>,
    pub const_attributes: &'const_attributes [AttributeDecl],
    pub item_const: &'item_const ItemConst,
}

#[derive(Clone)]
pub struct StorageContext<'module, 'item, 'attributes, 'item_storage> {
    pub path: &'module Path,
    pub module: &'module Module,
    pub item: &'item ItemKind,
    pub attributes: &'attributes [AttributeDecl],
    pub item_storage: &'item_storage ItemStorage,
}

#[derive(Clone)]
pub struct StorageFieldContext<'module, 'item, 'storage_attributes, 'item_storage, 'field_attributes, 'field> {
    pub path: &'module Path,
    pub module: &'module Module,
    pub item: &'item ItemKind,
    pub storage_attributes: &'storage_attributes [AttributeDecl],
    pub item_storage: &'item_storage ItemStorage,
    pub field_attributes: &'field_attributes [AttributeDecl],
    pub field: &'field StorageField,
}

#[derive(Clone)]
pub struct ConfigurableContext<'module, 'item, 'attributes, 'item_configurable> {
    pub path: &'module Path,
    pub module: &'module Module,
    pub item: &'item ItemKind,
    pub attributes: &'attributes [AttributeDecl],
    pub item_configurable: &'item_configurable ItemConfigurable,
}

#[derive(Clone)]
pub struct ConfigurableFieldContext<'module, 'item, 'configurable_attributes, 'item_configurable, 'field_attributes, 'field> {
    pub path: &'module Path,
    pub module: &'module Module,
    pub item: &'item ItemKind,
    pub configurable_attributes: &'configurable_attributes [AttributeDecl],
    pub item_configurable: &'item_configurable ItemConfigurable,
    pub field_attributes: &'field_attributes [AttributeDecl],
    pub field: &'field ConfigurableField,
}

#[derive(Clone)]
pub struct TypeAliasContext<'module, 'item, 'attributes, 'item_type_alias> {
    pub path: &'module Path,
    pub module: &'module Module,
    pub item: &'item ItemKind,
    pub attributes: &'attributes [AttributeDecl],
    pub item_type_alias: &'item_type_alias ItemTypeAlias,
}

#[allow(unused_variables)]
pub trait AstVisitor {
    fn visit_module(&mut self, context: &ModuleContext, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_module(&mut self, context: &ModuleContext, project: &mut Project) -> Result<(), Error> { Ok(()) }

    fn visit_module_item(&mut self, context: &ModuleItemContext, project: &mut Project) -> Result<(), Error> { Ok(()) }
    fn leave_module_item(&mut self, context: &ModuleItemContext, project: &mut Project) -> Result<(), Error> { Ok(()) }

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
            let context = ModuleItemContext {
                path: context.path,
                module: context.module,
                attributes: item.attribute_list.as_slice(),
                item: &item.value,
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

    fn visit_module_item(&mut self, context: &ModuleItemContext, project: &mut Project) -> Result<(), Error> {
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
                    attributes: context.attributes,
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
                    const_attributes: context.attributes,
                    item_const,
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

    fn leave_module_item(&mut self, context: &ModuleItemContext, project: &mut Project) -> Result<(), Error> {
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
        
        let mut visit_field = |field: &Annotated<TypeField>| -> Result<(), Error> {
            let context = StructFieldContext {
                path: context.path,
                module: context.module,
                item: context.item,
                struct_attributes: context.attributes,
                item_struct: context.item_struct,
                field_attributes: field.attribute_list.as_slice(),
                field: &field.value,
            };

            self.visit_struct_field(&context, project)?;
            self.leave_struct_field(&context, project)?;

            Ok(())
        };

        for field in context.item_struct.fields.inner.value_separator_pairs.iter() {
            visit_field(&field.0)?;
        }

        if let Some(field) = context.item_struct.fields.inner.final_value_opt.as_ref() {
            visit_field(field)?;
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
        
        let mut visit_field = |field: &Annotated<TypeField>| -> Result<(), Error> {
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

            Ok(())
        };

        for field in context.item_enum.fields.inner.value_separator_pairs.iter() {
            visit_field(&field.0)?;
        }

        if let Some(field) = context.item_enum.fields.inner.final_value_opt.as_ref() {
            visit_field(field)?;
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

        for statement in context.item_fn.body.inner.statements.iter() {
            let context = StatementContext {
                path: context.path,
                module: context.module,
                item: context.item,
                impl_attributes: context.impl_attributes,
                item_impl: context.item_impl,
                fn_attributes: context.fn_attributes,
                item_fn: context.item_fn,
                statement,
            };

            self.visit_statement(&context, project)?;
            self.leave_statement(&context, project)?;
        }

        if let Some(expr) = context.item_fn.body.inner.final_expr_opt.as_ref() {
            let context = ExprContext {
                path: context.path,
                module: context.module,
                item: context.item,
                impl_attributes: context.impl_attributes,
                item_impl: context.item_impl,
                fn_attributes: context.fn_attributes,
                item_fn: context.item_fn,
                expr,
            };

            self.visit_expr(&context, project)?;
            self.leave_expr(&context, project)?;
        }
        
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
                    statement: context.statement,
                    statement_let,
                };

                self.visit_statement_let(&context, project)?;
                self.leave_statement_let(&context, project)?;
            }

            Statement::Item(_) => {
                todo!("Can statements really be items?")
            }

            Statement::Expr { expr, .. } => {
                let context = ExprContext {
                    path: context.path,
                    module: context.module,
                    item: context.item,
                    impl_attributes: context.impl_attributes,
                    item_impl: context.item_impl,
                    fn_attributes: context.fn_attributes,
                    item_fn: context.item_fn,
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
        
        Ok(())
    }

    fn leave_expr(&mut self, context: &ExprContext, project: &mut Project) -> Result<(), Error> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_expr(context, project)?;
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
                        const_attributes: item.attribute_list.as_slice(),
                        item_const,
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

        for field in context.item_storage.fields.inner.value_separator_pairs.iter() {
            visit_field(&field.0)?;
        }

        if let Some(field) = context.item_storage.fields.inner.final_value_opt.as_ref() {
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

        for field in context.item_configurable.fields.inner.value_separator_pairs.iter() {
            visit_field(&field.0)?;
        }

        if let Some(field) = context.item_configurable.fields.inner.final_value_opt.as_ref() {
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
