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

| Color | Severity |
|-|-|
| 🔴 | High |
| 🟡 | Medium |
| 🟢 | Low |

|  | Name | Description |
|-|-|-|
| 🔴 | `arbitrary_asset_transfer` | Checks for functions that transfer native assets to an arbitrary address without access restriction. |
| 🔴 | `arbitrary_code_execution` | Checks for functions that make use of the `LDC` assembly instruction without access restriction. |
| 🟢 | `boolean_comparisons` | Checks if an expression contains a comparison with a boolean literal, which is unnecessary. |
| 🔴 | `discarded_assignments` | Checks for variables that are assigned to without being utilized. |
| 🟢 | `division_before_multiplication` | Checks for division operations before multiplications, which can result in value truncation. |
| 🟢 | `explicit_return_statements` | Checks for functions that end with explicit `return` statements, which is unnecessary. |
| 🟡 | `external_calls_in_loop` | Checks if any functions contain any loops which performs calls to external functions. |
| 🟡 | `inline_assembly_usage` | Checks functions for inline assembly usage. |
| 🟢 | `large_literals` | Checks for expressions that contain large literal values, which may be difficult to read or interpreted incorrectly. |
| 🟢 | `magic_numbers` | Checks for expressions that contain irregular numerical constants that can be introduced as named constants. |
| 🟡 | `missing_logs` | Checks for publicly-accessible functions that make changes to storage variables without emitting logs. |
| 🟡 | `msg_amount_in_loop` | Checks for calls to `std::context::msg_amount()` or `std::registers::balance()` inside a while loop. In most cases, the result of the call should be stored in a local variable and decremented over each loop iteration. |
| 🟢 | `non_zero_identity_validation` | Checks to see if functions containing `Identity`, `Address` and `ContractId` parameters are checked for a zero value. |
| 🔴 | `potential_infinite_loops` | Checks for potentially infinite loops. |
| 🟡 | `redundant_storage_access` | Checks for redundant calls to `storage.x.read()` and `storage.x.write(x)`. |
| 🟢 | `storage_field_mutability` | Checks for any storage fields that can be refactored into constants or configurable fields. |
| 🔴 | `storage_not_updated` | Checks for local variables that are read from storage, then modified without being written back to storage. |
| 🟢 | `storage_read_in_loop_condition` | Checks for loops that contain a storage read in their condition, which can increase gas costs for each iteration. |
| 🟢 | `unchecked_call_payload` | Checks for functions that supply a `raw_ptr` argument to the `CALL` assembly instruction, or a `Bytes` argument without checking its length. |
| 🔴 | `unprotected_initialization` | Checks for initializer functions that can be called without requirements. |
| 🔴 | `unprotected_storage_variables` | Checks for functions that make changes to storage variables without access restriction. |
| 🟡 | `unsafe_timestamp_usage` | Checks for dependence on `std::block::timestamp` or `std::block::timestamp_of_block`, which can be manipulated by an attacker. |
| 🟢 | `unused_imports` | Checks for imported symbols that are not used. |
| 🟡 | `weak_prng` | Checks for weak PRNG due to a modulo operation on a block timestamp. |
