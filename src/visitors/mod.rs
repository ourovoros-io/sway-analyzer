mod msg_amount_in_loop;
mod storage_not_updated;
mod visitor;
mod discarded_assignments;

pub use self::visitor::*;

use self::{
    msg_amount_in_loop::*, storage_not_updated::*, discarded_assignments::*,
};

type VisitorConstructor = fn() -> Box<dyn AstVisitor>;
type VisitorEntry = (&'static str, VisitorConstructor);

pub const VISITOR_TYPES: &[VisitorEntry] = &[
    ("msg_amount_in_loop", || Box::new(MsgAmountInLoopVisitor::default())),
    ("storage_not_updated", || Box::new(StorageNotUpdatedVisitor::default())),
    ("discarded_assignments", || Box::new(DiscardedAssignmentsVisitor::default())),
];
