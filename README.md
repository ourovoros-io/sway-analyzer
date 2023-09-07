# ![](assets/banner.png)

A security-focused static analyzer for Sway written in Rust. The tool makes use of the existing [`sway-ast`](https://github.com/FuelLabs/sway/tree/master/sway-ast) and [`sway-parse`](https://github.com/FuelLabs/sway/tree/master/sway-parse) crates in order to parse Sway source code into its abstract syntax tree (AST). A recursive AST visitor is implemented on top of this, which will walk the AST structures top-down in a context-sensitive manner. Detectors leverage the AST visitor in order to implement their logic by inspecting the values contained in certain parts of the AST structures.

## Requirements

The `sway-analyzer` binary requires the Rust compiler and its package manager, cargo.

See the [Rust Install](https://www.rust-lang.org/tools/install) page for installation options.

## Installation

The `sway-analyzer` binary can be installed using the following commands:

```bash
cd /path/to/sway-analyzer/
cargo install --path .
```

The `sway-analyzer` binary can be uninstalled using the following command:

```bash
cargo uninstall sway-analyzer
```

## Usage

`sway-analyzer [OPTIONS]`

| Flags | |
|-|-|
| `-h`, `--help` | Prints help information |
| `-V`, `--version` | Prints version information |

| Options | |
|-|-|
| `--detectors <detectors>...` | The specific detectors to utilize. (Optional; Leave unused for all) |
| `--directory <directory>` | The path to the Forc project directory. (Optional) |
| `--display-format <display-format>` | The display format of the report. Can be "Text" or "Json". (Default = Text) |
| `--files <files>...` | The paths to the Sway source files. (Optional) |
| `--sorting <sorting>` | The order to sort report entries by. Can be "Line" or "Severity". (Default = Line) |

## Detectors

| Name | Description |
|-|-|
| `boolean_comparisons` | Checks if an expression contains a comparison with a boolean literal, which is unnecessary. |
| `discarded_assignments` | Checks for variables that are assigned to without being utilized. |
| `division_before_multiplication` | Checks for division operations before multiplications, which can result in value truncation. |
| `external_calls_in_loop` | Checks if any functions contain any loops which performs calls to external functions. |
| `inline_assembly_usage` | Checks functions for inline assembly usage. |
| `input_identity_validation` | Checks to see if `Identity`, `Address` and `ContractId` parameters are checked for a zero value. |
| `large_literals` | Checks for expressions that contain large literal values, which may be difficult to read or interpreted incorrectly. |
| `missing_logs` | Checks for publicly-accessible functions that make changes to storage variables without emitted logs. |
| `msg_amount_in_loop` | Checks for calls to `std::context::msg_amount()` or `std::registers::balance()` inside a while loop. In most cases, the result of the call should be stored in a local variable and decremented over each loop iteration. |
| `potential_infinite_loops` | Checks for potentially infinite loops. |
| `redundant_storage_access` | Checks for redundant calls to `storage.x.read()` and `storage.x.write(x)`. |
| `storage_field_mutability` | Checks for any storage fields that can be refactored into constants or configurable fields. |
| `storage_not_updated` | Checks for local variables that are read from storage, then modified without being written back to storage. |
| ~~`unprotected_storage_variables`~~ (WIP) | Checks for functions that make changes to storage variables without access restriction. |
| `unsafe_timestamp_usage` | Checks for dependence on `std::block::timestamp` or `std::block::timestamp_of_block`, which can be manipulated by an attacker. |
| `unused_imports` | Checks for imported symbols that are not used. |
| `weak_prng` | Checks for weak PRNG due to a modulo operation on a block timestamp. |
