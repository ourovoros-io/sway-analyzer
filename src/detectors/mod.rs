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
    ("arbitrary_asset_transfer", || Box::<ArbitraryAssetTransferVisitor>::default()),
    ("arbitrary_code_execution", || Box::<ArbitraryCodeExecutionVisitor>::default()),
    ("boolean_comparison", || Box::<BooleanComparisonVisitor>::default()),
    ("discarded_assignment", || Box::<DiscardedAssignmentVisitor>::default()),
    ("division_before_multiplication", || Box::<DivisionBeforeMultiplicationVisitor>::default()),
    ("explicit_return_statement", || Box::<ExplicitReturnStatementVisitor>::default()),
    ("external_call_in_loop", || Box::<ExternalCallInLoopVisitor>::default()),
    ("inline_assembly_usage", || Box::<InlineAssemblyUsageVisitor>::default()),
    ("large_literal", || Box::<LargeLiteralVisitor>::default()),
    ("locked_native_asset", || Box::<LockedNativeAssetVisitor>::default()),
    ("magic_number", || Box::<MagicNumberVisitor>::default()),
    ("manipulatable_balance_usage", || Box::<ManipulatableBalanceUsageVisitor>::default()),
    ("missing_logs", || Box::<MissingLogsVisitor>::default()),
    ("msg_amount_in_loop", || Box::<MsgAmountInLoopVisitor>::default()),
    ("non_zero_identity_validation", || Box::<NonZeroIdentityValidationVisitor>::default()),
    ("potential_infinite_loop", || Box::<PotentialInfiniteLoopVisitor>::default()),
    ("redundant_comparison", || Box::<RedundantComparisonVisitor>::default()),
    ("redundant_storage_access", || Box::<RedundantStorageAccessVisitor>::default()),
    ("storage_field_mutability", || Box::<StorageFieldMutabilityVisitor>::default()),
    ("storage_not_updated", || Box::<StorageNotUpdatedVisitor>::default()),
    ("storage_read_in_loop_condition", || Box::<StorageReadInLoopConditionVisitor>::default()),
    ("strict_equality", || Box::<StrictEqualityVisitor>::default()),
    ("unchecked_call_payload", || Box::<UncheckedCallPayloadVisitor>::default()),
    ("unprotected_initialization", || Box::<UnprotectedInitializationVisitor>::default()),
    ("unprotected_storage_variable", || Box::<UnprotectedStorageVariableVisitor>::default()),
    ("unsafe_timestamp_usage", || Box::<UnsafeTimestampUsageVisitor>::default()),
    ("unused_import", || Box::<UnusedImportVisitor>::default()),
    ("weak_prng", || Box::<WeakPrngVisitor>::default()),
];
