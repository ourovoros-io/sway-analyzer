use std::path::PathBuf;

#[derive(Debug)]
pub enum Error {
    Wrapped(Box<dyn std::error::Error>),
    InvalidDisplayFormat(String),
    ParseFailed(PathBuf),
    FileNotFound(PathBuf),
    LineNotFound(PathBuf, usize),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Wrapped(e) => write!(f, "{e}"),
            Error::InvalidDisplayFormat(format) => write!(f, "Invalid display format: {format}"),
            Error::ParseFailed(path) => write!(f, "Failed to parse file: \"{}\"", path.to_string_lossy()),
            Error::FileNotFound(path) => write!(f, "File not found: \"{}\"", path.to_string_lossy()),
            Error::LineNotFound(path, offset) => write!(f, "Offset {offset} not found in file: \"{}\"", path.to_string_lossy()),
        }
    }
}

impl std::error::Error for Error {}
