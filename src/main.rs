pub mod detectors;
pub mod error;
pub mod project;
pub mod report;
pub mod utils;
pub mod visitor;

use error::Error;
use project::{DisplayFormat, Project};
use report::Sorting;
use std::{collections::HashSet, path::PathBuf};
use structopt::{clap::AppSettings, StructOpt};

#[derive(Default, StructOpt)]
#[structopt(global_settings = &[AppSettings::ColoredHelp, AppSettings::ArgRequiredElseHelp])]
struct Options {
    /// The display format of the report. Can be "Text" or "Json". (Default = Text)
    #[structopt(long)]
    display_format: Option<DisplayFormat>,

    /// The order to sort report entries by. Can be "Line" or "Severity". (Default = Line)
    #[structopt(long)]
    sorting: Option<Sorting>,

    /// The path to the Forc project directory. (Optional)
    #[structopt(long)]
    directory: Option<PathBuf>,

    /// The paths to the Sway source files. (Optional)
    #[structopt(long)]
    files: Vec<PathBuf>,

    /// The specific detectors to utilize. (Optional; Leave unused for all)
    #[structopt(long)]
    detectors: Vec<String>,

    /// The severity levels to `include` in the report. Default is all of the severities and options are `low` `medium` and `high` (Optional)
    #[structopt(long)]
    include: Vec<String>,

    /// The severity levels to `exclude` from the report. Default is none of the severities and options are `low` `medium` and `high` (Optional)
    #[structopt(long)]
    exclude: Vec<String>,
}

fn main() -> Result<(), Error> {
    let mut options = Options::from_args();

    // Make sure that we have a valid include or exclude input. Both are not allowed.
    if !options.include.is_empty() && !options.exclude.is_empty() {
        return Err(Error::Wrapped("Cannot use both include and exclude options.".into()));
    }

    // Make sure directory is a directory path
    if let Some(directory) = options.directory.as_mut() {
        let dir_string = directory.to_string_lossy();

        if !dir_string.ends_with('/') || !dir_string.ends_with('\\') {
            *directory = PathBuf::from(format!("{dir_string}/"));
        }
    }

    // Make sure that we have a target to analyze (either a directory or files)
    if options.directory.is_none() && options.files.is_empty() {
        return Err(Error::Wrapped("No directory or files provided as a target to analyze.".into()));
    }

    // Construct the project and analyze the modules
    let mut project = Project::try_from(&options)?;
    project.analyze_modules()?;

    // Filter the entries based on the include or exclude options
    let entries  = filter_entries(&project.report.borrow(), &options);
    project.report.borrow_mut().entries = entries.into_iter().collect();

    println!("{project}");

    Ok(())
}

/// Filter the entries based on the include or exclude options
fn filter_entries(report: &crate::report::Report, options: &Options) -> Vec<(PathBuf, Vec<crate::report::Entry>)> {
    let mut out = vec![];
    if !options.include.is_empty() {
        let include = &options.include[0];
        let filter_items: HashSet<_> = if include.contains(',') {
            include.split(',').map(str::to_ascii_lowercase).collect()
        } else {
            options.include.iter().map(|x| x.to_ascii_lowercase()).collect()
        };
        for (path, entry) in &report.entries {
            let filtered: Vec<_> = entry.iter().filter(|e| filter_items.contains(&e.severity.to_string().to_ascii_lowercase())).cloned().collect();
            if !filtered.is_empty() {
                out.push((path.clone(), filtered));
            }
        }
    } else if !options.exclude.is_empty() {
        let exclude = &options.exclude[0];
        let filter_items: HashSet<_> = if exclude.contains(',') {
            exclude.split(',').map(str::to_ascii_lowercase).collect()
        } else {
            options.exclude.iter().map(|x| x.to_ascii_lowercase()).collect()
        };
        for (path, entry) in &report.entries {
            let remaining: Vec<_> = entry.iter().filter(|e| !filter_items.contains(&e.severity.to_string().to_ascii_lowercase())).cloned().collect();
            if !remaining.is_empty() {
                out.push((path.clone(), remaining));
            }
        }
    }
    out
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn test_detector(name: &str, entry_count: usize) {
        let options = Options {
            directory: Some(format!("test/{name}").into()),
            detectors: vec![name.to_string()],
            ..Default::default()
        };
    
        let mut project = Project::try_from(&options).unwrap();
        project.analyze_modules().unwrap();
    
        println!("{project}");
    
        let mut actual_entry_count = 0;
    
        for (_, entries) in project.report.borrow().entries.iter() {
            actual_entry_count += entries.len();
        }
    
        if entry_count != actual_entry_count {
            panic!(
                "Expected {entry_count} {}, found {actual_entry_count} {}",
                if entry_count == 1 { "entry" } else { "entries" },
                if actual_entry_count == 1 { "entry" } else { "entries" },
            );
        }
    }

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

    #[test]
    fn test_include_detectors() {
        // The arbitrary_asset_transfer directory contains 15 low severity entries
        // and 16 high severity entries. We should see all 31 entries
        let options = Options {
            directory: Some(PathBuf::from("test/arbitrary_asset_transfer")),
            include: vec!["low".to_string(), "high".to_string()],
            ..Default::default()
        };

        let mut project = Project::try_from(&options).unwrap();
        project.analyze_modules().unwrap();

        // Filter the entries based on the include or exclude options
        let entries  = filter_entries(&project.report.borrow(), &options);
        project.report.borrow_mut().entries = entries.into_iter().collect();

        assert_eq!(project.report.borrow().entries[0].1.len(), 31);

        println!("{project}");
    }

    #[test]
    fn test_include_single_detectors() {
        // The arbitrary_asset_transfer directory contains 15 low severity entries
        // and 16 high severity entries. We should only see the low severity entries
        let options = Options {
            directory: Some(PathBuf::from("test/arbitrary_asset_transfer")),
            include: vec!["low".to_string()],
            ..Default::default()
        };

        let mut project = Project::try_from(&options).unwrap();
        project.analyze_modules().unwrap();

        // Filter the entries based on the include or exclude options
        let entries  = filter_entries(&project.report.borrow(), &options);
        project.report.borrow_mut().entries = entries.into_iter().collect();

        assert_eq!(project.report.borrow().entries[0].1.len(), 15);

        println!("{project}");
    }

    #[test]
    fn test_exclude_detectors() {
        // The arbitrary_asset_transfer directory contains 15 low severity entries
        // and 16 high severity entries. We should not see any entries
        let options = Options {
            directory: Some(PathBuf::from("test/arbitrary_asset_transfer")),
            exclude: vec!["low".to_string(), "high".to_string()],
            ..Default::default()
        };

        let mut project = Project::try_from(&options).unwrap();
        project.analyze_modules().unwrap();

        // Filter the entries based on the include or exclude options
        let entries  = filter_entries(&project.report.borrow(), &options);
        project.report.borrow_mut().entries = entries.into_iter().collect();

        assert_eq!(project.report.borrow().entries.len(), 0);

        println!("{project}");
    }

    #[test]
    fn test_exclude_single_detectors() {
        // The arbitrary_asset_transfer directory contains 15 low severity entries
        // and 16 high severity entries. We should only see the high severity entries
        let options = Options {
            directory: Some(PathBuf::from("test/arbitrary_asset_transfer")),
            exclude: vec!["low".to_string()],
            ..Default::default()
        };

        let mut project = Project::try_from(&options).unwrap();
        project.analyze_modules().unwrap();

        // Filter the entries based on the include or exclude options
        let entries  = filter_entries(&project.report.borrow(), &options);
        project.report.borrow_mut().entries = entries.into_iter().collect();

        assert_eq!(project.report.borrow().entries[0].1.len(), 16);

        println!("{project}");
    }
}
