// Enum workaround
use std::string::ToString;
// Standard libs
use std::env;
use std::path::Path;
// Error Handling
use miette::{Error, IntoDiagnostic, Result};

#[derive(Default, Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
/// Internal values to browse the fs
pub struct Gate {
    /// Config file dir path and file path if founded.
    pub directory_path: Option<String>,
    pub file_path: Option<String>,
}
impl Gate {
    pub fn file(&mut self, file_path: String) -> Result<Self> {
        let path = Path::new(&file_path);
        self.file_path = Some(path.display().to_string());
        self.directory_path = Some(path.parent().unwrap().display().to_string());
        Ok(self.to_owned())
    }
    pub fn directory(&mut self, file_path: String) -> Result<Self> {
        let path = Path::new(&file_path);
        self.directory_path = Some(path.display().to_string());
        Ok(self.to_owned())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Portal {
    pub seed: Option<String>,
    /// Process origin path and current path
    pub origin: Gate,
    pub current: Gate,
    pub target: Gate,
}
impl Default for Portal {
    fn default() -> Self {
        Portal {
            origin: Gate::default(),
            current: Gate::default(),
            target: Gate::default(),
            seed: None,
        }
    }
}
impl Portal {
    // Hydrate a default portal with current env
    pub fn new() -> Result<Self> {
        Ok(Portal {
            target: Gate::default(),
            origin: Gate::default().directory(env::current_dir().unwrap().display().to_string())?,
            current: Gate::default()
                .directory(env::current_dir().unwrap().display().to_string())?,
            seed: None,
        })
    }
}
