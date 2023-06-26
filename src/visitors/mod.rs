mod msg_amount_in_loop;
mod storage_not_updated;
mod visitor;
mod write_after_write;

pub use self::visitor::*;

use self::{
    msg_amount_in_loop::*, storage_not_updated::*, write_after_write::*,
};

type VisitorConstructor = fn() -> Box<dyn AstVisitor>;
type VisitorEntry = (&'static str, VisitorConstructor);

pub const VISITOR_TYPES: &[VisitorEntry] = &[
    ("msg_amount_in_loop", || Box::new(MsgAmountInLoopVisitor::default())),
    ("storage_not_updated", || Box::new(StorageNotUpdatedVisitor::default())),
    ("write_after_write", || Box::new(WriteAfterWriteVisitor::default())),
];
