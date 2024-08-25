// Struct
use crate::teleport::types::Gate;
use pipelight_error::PipelightError;
// Filesystem
use std::path::Path;
// Error Handling
use miette::Result;

impl Gate {
    /**
    Set the Gate directory_path and file_path internal values
    according to the provided file path
    */
    pub fn file(&mut self, file_path: String) -> Result<Self, PipelightError> {
        let path = Path::new(&file_path);
        self.file_path = Some(path.display().to_string());
        self.directory_path = Some(path.parent().unwrap().display().to_string());
        Ok(self.to_owned())
    }
    /**
    Set the Gate directory_path internal values
    according to the provided directory path
    */
    pub fn directory(&mut self, file_path: String) -> Result<Self, PipelightError> {
        let path = Path::new(&file_path);
        self.directory_path = Some(path.display().to_string());
        Ok(self.to_owned())
    }
}
