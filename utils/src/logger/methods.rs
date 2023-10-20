// Struct
use crate::logger::types::{LogFile, Logger};
use log::{trace, LevelFilter};
use uuid::Uuid;
// Fylesystem
use std::fs;
// Error Handling
use miette::{IntoDiagnostic, Result};

impl Logger {
    pub fn set_internal_level(&mut self, level: &LevelFilter) -> Result<Self> {
        self.internals.level = level.to_owned();
        self.update()?;
        Ok(self.to_owned())
    }
    pub fn set_level(&mut self, level: &LevelFilter) -> Result<Self> {
        self.pipelines.level = level.to_owned();
        self.update()?;
        Ok(self.to_owned())
    }

    /**
    Create a log file with given uuid to log pipeline with:

    let json = serde_json::to_string(&pipeline).unwrap();
    error!(target: "pipelines_to_file","{}", json);

    */
    pub fn set_file(&mut self, uuid: &Uuid) -> Self {
        self.pipelines.file_info = Some(LogFile {
            name: uuid.to_string(),
            ..self.pipelines.file_info.clone().unwrap()
        });
        self.update().unwrap();
        self.to_owned()
    }
    /**
    Delete logs directories
    */
    pub fn force_clean(&self) -> Result<()> {
        let file_info = &self.pipelines.file_info;
        if let Some(file_info) = file_info {
            let dir = file_info.directory.clone();
            fs::remove_dir_all(&dir).into_diagnostic()?;
            let message = format!("Hard deleted log directory: {}", dir);
            trace!("{}", message);
        };
        let file_info = &self.pipelines.file_info;
        if let Some(file_info) = file_info {
            let dir = file_info.directory.clone();
            fs::remove_dir_all(&dir).into_diagnostic()?;
            let message = format!("Hard deleted internal log directory: {}", dir);
            trace!("{}", message);
        };
        Ok(())
    }
}
