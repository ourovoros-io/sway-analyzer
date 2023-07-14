use crate::{
    detectors::*,
    error::Error,
    report::Report,
    visitor::*,
    Options
};
use std::{
    cell::RefCell,
    collections::HashMap,
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};
use sway_ast::Module;
use sway_types::Span;

#[derive(Default)]
pub struct Project {
    line_ranges: HashMap<PathBuf, Vec<(usize, usize)>>,
    modules: Rc<RefCell<HashMap<PathBuf, Module>>>,
    detectors: Rc<RefCell<AstVisitorRecursive>>,
    pub report: Rc<RefCell<Report>>,
}

impl TryFrom<&Options> for Project {
    type Error = Error;

    fn try_from(options: &Options) -> Result<Self, Self::Error> {
        let mut project = Project {
            report: Rc::new(RefCell::new(Report::default())),
            ..Default::default()
        };
    
        if let Some(path) = options.directory.as_ref() {
            if !path.is_dir() || !path.exists() {
                // TODO
            }
    
            fn parse_dir<P: AsRef<Path>>(project: &mut Project, path: P) -> Result<(), Error> {
                for entry in path.as_ref().read_dir().map_err(|e| Error::Wrapped(Box::new(e)))? {
                    let Ok(entry) = entry else { continue };
                    let path = entry.path();
    
                    let forc_toml_path = PathBuf::from(format!("{}Forc.toml", path.to_string_lossy()));
    
                    if forc_toml_path.is_file() && forc_toml_path.exists() {
                        let src_path = PathBuf::from(format!("{}src", path.to_string_lossy()));
                
                        if src_path.is_dir() && src_path.exists() {
                            parse_dir(project, src_path)?;
                            continue;
                        }    
                    }
            
                    if path.is_dir() {
                        parse_dir(project, path)?;
                    } else if path.is_file() && path.extension().map(|x| x == "sw").unwrap_or(false) {
                        project.parse_file(path)?;
                    }
                }
    
                Ok(())
            }
    
            parse_dir(&mut project, path)?;
        }
    
        for path in options.files.iter() {
            project.parse_file(path)?;
        }
    
        for &(detector_name, create_detector) in DETECTOR_TYPES {
            if options.detectors.is_empty() || options.detectors.iter().any(|v| v == detector_name) {
                project.detectors.borrow_mut().visitors.push(create_detector());
            }
        }
    
        Ok(project)
    }
}

impl Project {
    /// Attempts to parse the file from the supplied `path`.
    pub fn parse_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Error> {
        let path = PathBuf::from(path.as_ref());
        let source = std::fs::read_to_string(path.clone()).map_err(|e| Error::Wrapped(Box::new(e)))?;
        
        self.load_line_ranges(path.clone(), source.as_str());

        let handler = sway_error::handler::Handler::default();
        let source = Arc::from(source.as_str());
        let module = sway_parse::parse_file(&handler, source, None).map_err(|_| Error::ParseFailed(path.clone()))?;

        self.modules.borrow_mut().insert(path, module.value);

        Ok(())
    }

    /// Loads line ranges in a specfic file `path` from the provided `source` text.
    fn load_line_ranges(&mut self, path: PathBuf, source: &str) {
        let mut line_range = (0usize, 0usize);

        for (i, c) in source.chars().enumerate() {
            if c == '\n' {
                line_range.1 = i;
                self.line_ranges.entry(path.clone()).or_insert(vec![]).push(line_range);
                line_range = (i + 1, 0);
            }
        }

        if line_range.1 > line_range.0 {
            self.line_ranges.entry(path.clone()).or_insert(vec![]).push(line_range);
        }
    }

    /// Attempts to get the line number in the supplied file `path` for the provided `span`.
    pub fn span_to_line(&self, path: &Path, span: &Span) -> Result<Option<usize>, Error> {
        let line_ranges = self.line_ranges.get(path.into()).ok_or_else(|| Error::FileNotFound(path.into()))?;
        let offset = span.start();

        if line_ranges.is_empty() {
            return Ok(None);
        }

        for (i, line_range) in line_ranges.iter().enumerate() {
            if offset >= line_range.0 && offset < line_range.1 {
                return Ok(Some(i + 1));
            }
        }

        Err(Error::LineNotFound(path.into(), offset))
    }

    /// Attempts to analyze all of the parsed files.
    pub fn analyze_modules(&mut self) -> Result<(), Error> {
        let modules = self.modules.clone();
        let detectors = self.detectors.clone();

        for (path, module) in modules.borrow().iter() {
            let context = ModuleContext {
                path,
                module,
            };

            detectors.borrow_mut().visit_module(&context, self)?;
            detectors.borrow_mut().leave_module(&context, self)?;
        }

        Ok(())
    }
}
