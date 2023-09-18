mod arbitrary_asset_transfer;
mod boolean_comparisons;
mod discarded_assignments;
mod division_before_multiplication;
mod explicit_return_statements;
mod external_calls_in_loop;
mod inline_assembly_usage;
mod input_identity_validation;
mod large_literals;
mod magic_numbers;
mod missing_logs;
mod msg_amount_in_loop;
mod potential_infinite_loops;
mod redundant_storage_access;
mod storage_field_mutability;
mod storage_not_updated;
mod storage_read_in_loop_condition;
mod unprotected_initialization;
mod unprotected_storage_variables;
mod unsafe_timestamp_usage;
mod unused_imports;
mod weak_prng;

use crate::visitor::AstVisitor;

use self::{
    arbitrary_asset_transfer::*, boolean_comparisons::*, discarded_assignments::*,
    division_before_multiplication::*, explicit_return_statements::*, external_calls_in_loop::*,
    inline_assembly_usage::*, input_identity_validation::*, large_literals::*, magic_numbers::*,
    missing_logs::*, msg_amount_in_loop::*, potential_infinite_loops::*,
    redundant_storage_access::*, storage_field_mutability::*, storage_not_updated::*,
    storage_read_in_loop_condition::*, unprotected_initialization::*,
    unprotected_storage_variables::*, unsafe_timestamp_usage::*, unused_imports::*, weak_prng::*,
};

type DetectorConstructor = fn() -> Box<dyn AstVisitor>;
type DetectorEntry = (&'static str, DetectorConstructor);

pub const DETECTOR_TYPES: &[DetectorEntry] = &[
    ("arbitrary_asset_transfer", || Box::new(ArbitraryAssetTransferVisitor::default())),
    ("boolean_comparisons", || Box::new(BooleanComparisonsVisitor::default())),
    ("discarded_assignments", || Box::new(DiscardedAssignmentsVisitor::default())),
    ("division_before_multiplication", || Box::new(DivisionBeforeMultiplicationVisitor::default())),
    ("explicit_return_statements", || Box::new(ExplicitReturnStatementsVisitor::default())),
    ("external_calls_in_loop", || Box::new(ExternalCallsInLoopVisitor::default())),
    ("inline_assembly_usage", || Box::new(InlineAssemblyUsageVisitor::default())),
    ("input_identity_validation", || Box::new(InputIdentityValidationVisitor::default())),
    ("large_literals", || Box::new(LargeLiteralsVisitor::default())),
    ("magic_numbers", || Box::new(MagicNumbersVisitor::default())),
    ("missing_logs", || Box::new(MissingLogsVisitor::default())),
    ("msg_amount_in_loop", || Box::new(MsgAmountInLoopVisitor::default())),
    ("potential_infinite_loops", || Box::new(PotentialInfiniteLoopsVisitor::default())),
    ("redundant_storage_access", || Box::new(RedundantStorageAccessVisitor::default())),
    ("storage_field_mutability", || Box::new(StorageFieldMutabilityVisitor::default())),
    ("storage_not_updated", || Box::new(StorageNotUpdatedVisitor::default())),
    ("storage_read_in_loop_condition", || Box::new(StorageReadInLoopConditionVisitor::default())),
    ("unprotected_initialization", || Box::new(UnprotectedInitializationVisitor::default())),
    ("unprotected_storage_variables", || Box::new(UnprotectedStorageVariablesVisitor::default())),
    ("unsafe_timestamp_usage", || Box::new(UnsafeTimestampUsageVisitor::default())),
    ("unused_imports", || Box::new(UnusedImportsVisitor::default())),
    ("weak_prng", || Box::new(WeakPrngVisitor::default())),
];
