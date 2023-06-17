mod visitor;
mod storage;

pub use self::visitor::*;

use self::{
    storage::*,
};

type VisitorConstructor = fn() -> Box<dyn AstVisitor>;
type VisitorEntry = (&'static str, VisitorConstructor);

pub const VISITOR_TYPES: &[VisitorEntry] = &[
    ("storage", || Box::new(StorageFieldsVisitor)),
];
