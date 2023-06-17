# sway-analyzer
A static analyzer for Sway written in Rust.

## Usage

`sway-analyzer [OPTIONS]`

| Flags | |
|-|-|
| `-h`, `--help` | Prints help information |
| `-V`, `--version` | Prints version information |

| Options | |
|-|-|
| `--directory <directory>` | The path to the Forc project directory. (Optional) |
| `--files <files>...` | The paths to the Sway source files. (Optional) |
| `--visitors <visitors>...` | The specific visitors to utilize. (Optional; Leave unused for all) |

## Visitors

| Name | Description |
|-|-|
| `storage` | An example visitor that prints the name of each storage field |
