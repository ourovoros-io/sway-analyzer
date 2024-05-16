use crate::{detectors::*, error::Error, report::Report, scope::AstScope, visitor::*, Options};
use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::Display,
    path::{Path, PathBuf},
    rc::Rc,
    str::FromStr,
    sync::Arc,
};
use sway_ast::Module;
use sway_ast_stubs::AstResolver;
use sway_types::Span;

#[derive(Clone, Copy, Default)]
pub enum DisplayFormat {
    #[default]
    Text,
    Json,
}

impl FromStr for DisplayFormat {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "text" => Ok(Self::Text),
            "json" => Ok(Self::Json),
            _ => Err(Error::InvalidDisplayFormat(s.to_string())),
        }
    }
}

#[derive(Default)]
pub struct Project<'a> {
    display_format: DisplayFormat,
    line_ranges: HashMap<PathBuf, Vec<(usize, usize)>>,
    modules: Rc<RefCell<HashMap<PathBuf, Module>>>,
    detectors: Rc<RefCell<AstVisitorRecursive<'a>>>,
    pub report: Rc<RefCell<Report>>,
    pub resolver: Rc<RefCell<AstResolver>>,
}

impl Display for Project<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.display_format {
            DisplayFormat::Text => {
                write!(f, "{}", self.report.borrow())?;
            }

            DisplayFormat::Json => {
                let value = serde_json::to_value(self.report.borrow().clone()).unwrap();
                write!(f, "{}", value)?;
            }
        }

        Ok(())
    }
}

impl TryFrom<&Options> for Project<'_> {
    type Error = Error;

    fn try_from(options: &Options) -> Result<Self, Self::Error> {
        let mut project = Project {
            display_format: options.display_format.unwrap_or_default(),
            report: Rc::new(RefCell::new(Report {
                sorting: options.sorting.unwrap_or_default(),
                ..Default::default()
            })),
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

        // Check if detectors are valid and if not, return an error with the detector name that is not valid.
        if !options.detectors.is_empty() {
            for detector in &options.detectors {
                if !DETECTOR_TYPES.iter().any(|(name, _)| detector == *name) {
                    return Err(Error::Wrapped(format!("Detector not found in detectors collection : {detector}").into()));
                }
            }
        }
    
        for &(detector_name, create_detector) in DETECTOR_TYPES {
            if options.detectors.is_empty() || options.detectors.iter().any(|v| v == detector_name) {
                project.detectors.borrow_mut().visitors.push(create_detector());
            }
        }
    
        Ok(project)
    }
}

impl Project<'_> {
    /// Attempts to parse the file from the supplied `path`.
    pub fn parse_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Error> {
        let path = PathBuf::from(path.as_ref().to_string_lossy().replace("\\\\", "\\").replace("//", "/"));
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
                self.line_ranges.entry(path.clone()).or_default().push(line_range);
                line_range = (i + 1, 0);
            }
        }

        if line_range.1 > line_range.0 {
            self.line_ranges.entry(path.clone()).or_default().push(line_range);
        }
    }

    /// Attempts to get the line number in the supplied file `path` for the provided `span`.
    pub fn span_to_line(&self, path: &Path, span: &Span) -> Result<Option<usize>, Error> {
        let line_ranges = self.line_ranges.get(path).ok_or_else(|| Error::FileNotFound(path.into()))?;
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

        let mut module_paths = modules.borrow().keys().cloned().collect::<Vec<_>>();
        module_paths.sort();

        for path in module_paths {
            println!("{}", path.to_string_lossy());
            
            let modules = modules.borrow();
            let module = modules.get(&path).unwrap();

            let context = ModuleContext {
                path: &path,
                module,
            };

            let scope = Rc::new(RefCell::new(AstScope::default()));

            detectors.borrow_mut().visit_module(&context, scope.clone(), self)?;
            detectors.borrow_mut().leave_module(&context, scope.clone(), self)?;
        }

        Ok(())
    }
}
