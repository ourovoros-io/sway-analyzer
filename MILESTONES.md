
# Milestones for `sway-analyzer`

## Table of Contents

* [Milestone 1: Basic Static Analysis Functionality](#milestone-1-basic-static-analysis-functionality)
* [Milestone 2: Detectors](#milestone-2-detectors)
    * [Milestone 2.1: `storage_not_updated` Detector](#milestone-21-storage_not_updated-detector)
    * [Milestone 2.2: `input_identity_validation` Detector](#milestone-22-input_identity_validation-detector)
    * [Milestone 2.3: `inline_assembly_usage` Detector](#milestone-23-inline_assembly_usage-detector)
    * [Milestone 2.4: `discarded_assignments` Detector](#milestone-24-discarded_assignments-detector)
    * [Milestone 2.5: `missing_logs` Detector](#milestone-25-missing_logs-detector)
    * [Milestone 2.6: `msg_amount_in_loops` Detector](#milestone-26-msg_amount_in_loops-detector)
    * Unsorted sub-milestones for Milestone 2:
        * [Milestone 2.X: `large_literals` Detector](#milestone-2x-large_literals-detector)
        * [Milestone 2.X: `division_before_multiplication` Detector](#milestone-2x-division_before_multiplication-detector)
        * [Milestone 2.X: `boolean_literals` Detector](#milestone-2x-boolean_literals-detector)
        * [Milestone 2.X: `unused_imports` Detector](#milestone-2x-unused_imports-detector)
        * [Milestone 2.X: `external_calls_in_loops` Detector](#milestone-2x-external_calls_in_loops-detector)
        * [Milestone 2.X: `secure_native_asset_transfer` Detector](#milestone-2x-secure_native_asset_transfer-detector)
        * [Milestone 2.X: `missing_withdraw_function` Detector](#milestone-2x-missing_withdraw_function-detector)
        * [Milestone 2.X: `incorrect_shift` Detector](#milestone-2x-incorrect_shift-detector)
        * [Milestone 2.X: `missing_require` Detector](#milestone-2x-missing_require-detector)
        * [Milestone 2.X: `redundant_comparisons` Detector](#milestone-2x-redundant_comparisons-detector)
        * [Milestone 2.X: `manipulatable_balance_usage` Detector](#milestone-2x-manipulatable_balance_usage-detector)
        * [Milestone 2.X: `redundant_imports` Detector](#milestone-2x-redundant_imports-detector)
        * [Milestone 2.X: `ineffectual_statements` Detector](#milestone-2x-ineffectual_statements-detector)
        * [Milestone 2.X: `comparison_utilization` Detector](#milestone-2x-comparison_utilization-detector)
        * [Milestone 2.X: `unpaid_payable_functions` Detector](#milestone-2x-unpaid_payable_functions-detector)
* [Milestone 3: Pretty Printers](#milestone-3-pretty-printers)
    * [Milestone 3.1: `storage_pretty_print` Printer](#milestone-31-storage_pretty_print-printer)
    * [Milestone 3.2: `function_signatures` Printer](#milestone-32-function_signatures-printer)
    * [Milestone 3.3: `src20_compliance` Printer](#milestone-33-src20_compliance-printer)
* [Milestone 4: Feedback Loop](#milestone-4-feedback-loop)

## Milestone 1: Basic Static Analysis Functionality

**Status:** Complete

* [x] Parse Sway files from individual files and folders into their respective module AST into a "project" object
* [x] Keep track of issues identified in specific Sway files on specific lines in a "report" object
* [x] Implement a top-down AST visitor which granularly exposes contents of each component of a module AST and
* [x] Implement a generic detector system which allows each security pattern to be isolated and looked up by name
* [x] Handle displaying of the "report" object in either plain text or JSON for integration with external tooling

## Milestone 2: Detectors

### Milestone 2.1: `storage_not_updated` Detector

**Status:** In Progress - Implementation complete; Needs test contract

Checks for local variables that are read from storage, then modified without being written back to storage.

### Milestone 2.2: `input_identity_validation` Detector

**Status:** Complete

Checks to see if `Identity`, `Address` and `ContractId` parameters are checked for a zero value.

### Milestone 2.3: `inline_assembly_usage` Detector

**Status:** In Progress - Implementation complete; Needs test contract

Checks functions for inline assembly usage.

### Milestone 2.4: `discarded_assignments` Detector

**Status:** In Progress

Checks for variables that are assigned to without being utilized.

### Milestone 2.5: `missing_logs` Detector

**Status:** In Progress - Implementation complete; Needs test contract

Checks for publicly-accessible functions that make changes to storage variables without emitted logs.

### Milestone 2.6: `msg_amount_in_loops` Detector

**Status:** In Progress - Implementation complete; Needs test contract

Checks for calls to `std::context::msg_amount()` or `std::registers::balance()` inside a while loop. In most cases, the result of the call should be stored in a local variable and decremented over each loop iteration.

---

**NOTE:** The milestones below need to be reviewed and prioritized in implementation order preference.

---

### Milestone 2.X: `large_literals` Detector

**Status:** In Progress

Determines if an expression contains a large literal value, which may be difficult to read or could be interpreted incorrectly.

### Milestone 2.X: `division_before_multiplication` Detector

**Status:** In Progress

Checks for division operations before multiplications, which can result in value truncation.

### Milestone 2.X: `boolean_literals` Detector

**Status:** In Progress

Determines if an expression contains a boolean literal value comparison.

### Milestone 2.X: `unused_imports` Detector

**Status:** Not Started

Checks for imported symbols which are not used.

### Milestone 2.X: `external_calls_in_loops` Detector

**Status:** Not Started

Determines if any functions or modifiers contain any loops which performs calls to external functions

### Milestone 2.X: `secure_native_asset_transfer` Detector

**Status:** Not Started

Determines if any functions do not use the TR opcode to transfer native assets.

### Milestone 2.X: `missing_withdraw_function` Detector

**Status:** Not Started

Checks for an exit point for the native asset stored in the contract.

### Milestone 2.X: `incorrect_shift` Detector

**Status:** Not Started

Determines if the values in a shift operation are reversed.

### Milestone 2.X: `missing_require` Detector

**Status:** Not Started

Checks whether input values are filtered.

### Milestone 2.X: `redundant_comparisons` Detector

**Status:** Not Started

Determines if any comparisons are redundant, tautology or contradiction, i.e: `true != false`.

### Milestone 2.X: `manipulatable_balance_usage` Detector

**Status:** Not Started

Determines if any functions or modifiers contain balance usage which can potentially be manipulated.

### Milestone 2.X: `redundant_imports` Detector

**Status:** Not Started

Determines if any import directives are redundant due to the specified path being already previously imported.

### Milestone 2.X: `ineffectual_statements` Detector

**Status:** Not Started

Determines if any statements are ineffectual.

### Milestone 2.X: `comparison_utilization` Detector

**Status:** Not Started

Determines if an if statement's condition contains a comparison without utilizing either compared value in its true or false branches.

### Milestone 2.X: `unpaid_payable_functions` Detector

**Status:** Not Started

Determines if any functions or modifiers perform calls to payable function without paying.

## Milestone 3: Pretty Printers

### Milestone 3.1: `storage_pretty_print` Printer

**Status:** Not Started

Displays a detailed storage layout for any specified contracts.

### Milestone 3.2: `function_signatures` Printer

**Status:** Not Started

Displays function signatures for any specified contracts.

### Milestone 3.3: `src20_compliance` Printer

**Status:** Not Started

Checks all specified contracts for SRC-20 compliance.

## Milestone 4: Feedback Loop

Get feedback from the Sway community and security experts in order to determine new detectors and feature requests.
