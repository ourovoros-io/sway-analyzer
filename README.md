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

## Detectors

| Name | Description |
|-|-|
| `discarded_assignments` | Checks for variables that are assigned to without being utilized. |
| `inline_assembly_usage` | Checks functions for inline assembly usage. |
| `input_identity_validation` | Checks to see if `Identity`, `Address` and `ContractId` parameters are checked for a zero value. |
| `missing_logs` | Checks for publicly-accessible functions that make changes to storage variables without emitted logs. |
| `msg_amount_in_loop` | Checks for calls to `std::context::msg_amount()` or `std::registers::balance()` inside a while loop. In most cases, the result of the call should be stored in a local variable and decremented over each loop iteration. |
| `storage_not_updated` | Checks for local variables that are read from storage, then modified without being written back to storage. |
