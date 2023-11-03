mod arbitrary_asset_transfer;
mod arbitrary_code_execution;
mod boolean_comparison;
mod discarded_assignment;
mod division_before_multiplication;
mod explicit_return_statement;
mod external_call_in_loop;
mod inline_assembly_usage;
mod large_literal;
mod locked_native_asset;
mod magic_number;
mod manipulatable_balance_usage;
mod missing_logs;
mod msg_amount_in_loop;
mod non_zero_identity_validation;
mod potential_infinite_loop;
mod redundant_comparison;
mod redundant_storage_access;
mod storage_field_mutability;
mod storage_not_updated;
mod storage_read_in_loop_condition;
mod strict_equality;
mod unchecked_call_payload;
mod unprotected_initialization;
mod unprotected_storage_variable;
mod unsafe_timestamp_usage;
mod unused_import;
mod weak_prng;

use crate::visitor::AstVisitor;

use self::{
    arbitrary_asset_transfer::*, arbitrary_code_execution::*, boolean_comparison::*,
    discarded_assignment::*, division_before_multiplication::*, explicit_return_statement::*,
    external_call_in_loop::*, inline_assembly_usage::*, large_literal::*, locked_native_asset::*,
    magic_number::*, manipulatable_balance_usage::*, missing_logs::*, msg_amount_in_loop::*,
    non_zero_identity_validation::*, potential_infinite_loop::*, redundant_comparison::*,
    redundant_storage_access::*, storage_field_mutability::*, storage_not_updated::*,
    storage_read_in_loop_condition::*, strict_equality::*, unchecked_call_payload::*,
    unprotected_initialization::*, unprotected_storage_variable::*, unsafe_timestamp_usage::*,
    unused_import::*, weak_prng::*,
};

type DetectorConstructor = fn() -> Box<dyn AstVisitor>;
type DetectorEntry = (&'static str, DetectorConstructor);

pub const DETECTOR_TYPES: &[DetectorEntry] = &[
    ("arbitrary_asset_transfer", || Box::new(ArbitraryAssetTransferVisitor::default())),
    ("arbitrary_code_execution", || Box::new(ArbitraryCodeExecutionVisitor::default())),
    ("boolean_comparison", || Box::new(BooleanComparisonVisitor::default())),
    ("discarded_assignment", || Box::new(DiscardedAssignmentVisitor::default())),
    ("division_before_multiplication", || Box::new(DivisionBeforeMultiplicationVisitor::default())),
    ("explicit_return_statement", || Box::new(ExplicitReturnStatementVisitor::default())),
    ("external_call_in_loop", || Box::new(ExternalCallInLoopVisitor::default())),
    ("inline_assembly_usage", || Box::new(InlineAssemblyUsageVisitor::default())),
    ("large_literal", || Box::new(LargeLiteralVisitor::default())),
    ("locked_native_asset", || Box::new(LockedNativeAssetVisitor::default())),
    ("magic_number", || Box::new(MagicNumberVisitor::default())),
    ("manipulatable_balance_usage", || Box::new(ManipulatableBalanceUsageVisitor::default())),
    ("missing_logs", || Box::new(MissingLogsVisitor::default())),
    ("msg_amount_in_loop", || Box::new(MsgAmountInLoopVisitor::default())),
    ("non_zero_identity_validation", || Box::new(NonZeroIdentityValidationVisitor::default())),
    ("potential_infinite_loop", || Box::new(PotentialInfiniteLoopVisitor::default())),
    ("redundant_comparison", || Box::new(RedundantComparisonVisitor::default())),
    ("redundant_storage_access", || Box::new(RedundantStorageAccessVisitor::default())),
    ("storage_field_mutability", || Box::new(StorageFieldMutabilityVisitor::default())),
    ("storage_not_updated", || Box::new(StorageNotUpdatedVisitor::default())),
    ("storage_read_in_loop_condition", || Box::new(StorageReadInLoopConditionVisitor::default())),
    ("strict_equality", || Box::new(StrictEqualityVisitor::default())),
    ("unchecked_call_payload", || Box::new(UncheckedCallPayloadVisitor::default())),
    ("unprotected_initialization", || Box::new(UnprotectedInitializationVisitor::default())),
    ("unprotected_storage_variable", || Box::new(UnprotectedStorageVariableVisitor::default())),
    ("unsafe_timestamp_usage", || Box::new(UnsafeTimestampUsageVisitor::default())),
    ("unused_import", || Box::new(UnusedImportVisitor::default())),
    ("weak_prng", || Box::new(WeakPrngVisitor::default())),
];
