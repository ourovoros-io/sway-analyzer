mod discarded_assignments;
mod missing_logs;
mod msg_amount_in_loop;
mod storage_not_updated;
mod visitor;

pub use self::visitor::*;

use self::{
    msg_amount_in_loop::*, storage_not_updated::*, discarded_assignments::*, missing_logs::MissingLogsVisitor,
};

type VisitorConstructor = fn() -> Box<dyn AstVisitor>;
type VisitorEntry = (&'static str, VisitorConstructor);

pub const VISITOR_TYPES: &[VisitorEntry] = &[
    ("discarded_assignments", || Box::new(DiscardedAssignmentsVisitor::default())),
    ("missing_logs", || Box::new(MissingLogsVisitor::default())),
    ("msg_amount_in_loop", || Box::new(MsgAmountInLoopVisitor::default())),
    ("storage_not_updated", || Box::new(StorageNotUpdatedVisitor::default())),
];
