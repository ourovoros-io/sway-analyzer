pub mod detectors;
pub mod error;
pub mod project;
pub mod report;
pub mod utils;
pub mod visitor;

use error::Error;
use project::{Project, DisplayFormat};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Default, StructOpt)]
struct Options {
    /// The display format of the report. Can be "Text" or "Json". (Default = Text)
    #[structopt(long)]
    display_format: DisplayFormat,

    /// The path to the Forc project directory. (Optional)
    #[structopt(long)]
    directory: Option<PathBuf>,

    /// The paths to the Sway source files. (Optional)
    #[structopt(long)]
    files: Vec<PathBuf>,

    /// The specific detectors to utilize. (Optional; Leave unused for all)
    #[structopt(long)]
    detectors: Vec<String>,
}

fn main() -> Result<(), Error> {
    let mut options = Options::from_args();

    // Make sure directory is a directory path
    if let Some(directory) = options.directory.as_mut() {
        let dir_string = directory.to_string_lossy();

        if !dir_string.ends_with('/') || !dir_string.ends_with('\\') {
            *directory = PathBuf::from(format!("{dir_string}/"));
        }
    }

    if options.directory.is_none() && options.files.is_empty() {
        // TODO: print help
        return Ok(());
    }

    let mut project = Project::try_from(&options)?;
    project.analyze_modules()?;

    println!("{project}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detectors() {
        let options = Options {
            directory: Some(PathBuf::from("test/")),
            ..Default::default()
        };

        let mut project = Project::try_from(&options).unwrap();
        project.analyze_modules().unwrap();

        println!("{project}");
    }
}
