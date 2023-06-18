mod visitor;
mod storage_not_updated;

pub use self::visitor::*;

use self::{
    storage_not_updated::*,
};

type VisitorConstructor = fn() -> Box<dyn AstVisitor>;
type VisitorEntry = (&'static str, VisitorConstructor);

pub const VISITOR_TYPES: &[VisitorEntry] = &[
    ("storage_not_updated", || Box::new(StorageNotUpdatedVisitor::default())),
];
