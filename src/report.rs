use crate::error::Error;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, path::PathBuf, str::FromStr};

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub enum Sorting {
    #[default]
    Line,
    Severity,
}

impl FromStr for Sorting {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "line" => Ok(Self::Line),
            "severity" => Ok(Self::Severity),
            _ => Err(Error::InvalidSorting(s.to_string())),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord)]
pub enum Severity {
    High,
    Medium,
    Low,
}

impl Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Entry {
    pub line: Option<usize>,
    pub severity: Severity,
    pub text: String,
}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let line = format!(
            "{}{}",
            if let Some(line) = self.line.as_ref() {
                format!("L{}; {}: ", line, self.severity)
            } else {
                format!("{}: ", self.severity)
            },
            self.text,
        );

        let output = match self.severity {
            Severity::High => line.red(),
            Severity::Medium => line.yellow(),
            Severity::Low => line.green(),
        };

        write!(f, "{output}")
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Report {
    pub sorting: Sorting,
    pub entries: Vec<(PathBuf, Vec<Entry>)>,
}

impl Report {
    pub fn add_entry<P: Into<PathBuf>, S: Into<String>>(
        &mut self,
        file: P,
        line: Option<usize>,
        severity: Severity,
        text: S,
    ) {
        let file: PathBuf = file.into();

        if !self.entries.iter().any(|(path, _)| file.eq(path)) {
            self.entries.push((file.clone(), vec![]));
            self.entries.sort_by(|(a, _), (b, _)| a.cmp(b));
        }

        let file_entry = self
            .entries
            .iter_mut()
            .find(|(path, _)| file.eq(path))
            .unwrap();

        file_entry.1.push(Entry {
            line,
            severity,
            text: text.into(),
        });

        match self.sorting {
            Sorting::Line => file_entry.1.sort_unstable_by_key(|x| (x.line, x.severity)),
            Sorting::Severity => file_entry.1.sort_unstable_by_key(|x| (x.severity, x.line)),
        }
    }
}

impl Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, (path, entries)) in self.entries.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }

            writeln!(f, "{}:", path.to_string_lossy())?;

            for entry in entries.iter() {
                writeln!(f, "\t{entry}")?;
            }
        }

        Ok(())
    }
}
