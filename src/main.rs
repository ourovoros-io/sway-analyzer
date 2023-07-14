pub mod detectors;
pub mod error;
pub mod project;
pub mod report;
pub mod utils;

use error::Error;
use project::Project;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Default, StructOpt)]
struct Options {
    /// The path to the Forc project directory. (Optional)
    #[structopt(long)]
    directory: Option<PathBuf>,

    /// The paths to the Sway source files. (Optional)
    #[structopt(long)]
    files: Vec<PathBuf>,

    /// The specific visitors to utilize. (Optional; Leave unused for all)
    #[structopt(long)]
    visitors: Vec<String>,
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

    println!("{}", project.report.borrow());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_from_directory() {
        let options = Options {
            directory: Some(PathBuf::from("test/test-contract/")),
            ..Default::default()
        };

        let mut project = Project::try_from(&options).unwrap();
        project.analyze_modules().unwrap();

        println!("{}", project.report.borrow());
    }

    #[test]
    fn test_project_from_files() {
        let options = Options {
            files: vec![PathBuf::from("test/test-contract/src/main.sw")],
            ..Default::default()
        };

        let mut project = Project::try_from(&options).unwrap();
        project.analyze_modules().unwrap();

        println!("{}", project.report.borrow());
    }
}
