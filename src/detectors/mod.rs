mod discarded_assignments;
mod external_calls_in_loop;
mod inline_assembly_usage;
mod input_identity_validation;
mod missing_logs;
mod msg_amount_in_loop;
mod redundant_storage_access;
mod storage_not_updated;

use crate::visitor::AstVisitor;

use self::{
    discarded_assignments::*, external_calls_in_loop::*, inline_assembly_usage::*,
    input_identity_validation::*, missing_logs::*, msg_amount_in_loop::*,
    redundant_storage_access::*, storage_not_updated::*,
};

type DetectorConstructor = fn() -> Box<dyn AstVisitor>;
type DetectorEntry = (&'static str, DetectorConstructor);

pub const DETECTOR_TYPES: &[DetectorEntry] = &[
    ("discarded_assignments", || Box::new(DiscardedAssignmentsVisitor::default())),
    ("external_calls_in_loop", || Box::new(ExternalCallsInLoopVisitor::default())),
    ("inline_assembly_usage", || Box::new(InlineAssemblyUsageVisitor::default())),
    ("input_identity_validation", || Box::new(InputIdentityValidationVisitor::default())),
    ("missing_logs", || Box::new(MissingLogsVisitor::default())),
    ("msg_amount_in_loop", || Box::new(MsgAmountInLoopVisitor::default())),
    ("redundant_storage_access", || Box::new(RedundantStorageAccessVisitor::default())),
    ("storage_not_updated", || Box::new(StorageNotUpdatedVisitor::default())),
];
