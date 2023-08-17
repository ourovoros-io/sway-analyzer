use serde::{Deserialize, Serialize};
use std::{fmt::Display, path::PathBuf};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Entry {
    pub line: Option<usize>,
    pub text: String,
}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            if let Some(line) = self.line.as_ref() {
                format!("L{}: ", line)
            } else {
                String::new()
            },
            self.text,
        )
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Report {
    pub entries: Vec<(PathBuf, Vec<Entry>)>,
}

impl Report {
    pub fn add_entry<P: Into<PathBuf>, S: Into<String>>(
        &mut self,
        file: P,
        line: Option<usize>,
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
            text: text.into(),
        });

        file_entry.1.sort_by(|a, b| a.line.cmp(&b.line));
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
