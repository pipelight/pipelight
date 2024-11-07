pub use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};
// Struct
use uuid::Uuid;
// Fylesystem
use std::{env, fs};
// Error Handling
use miette::{IntoDiagnostic, Result};

// Re-export

#[derive(Debug, Clone)]
pub struct Logger {
    pub pipelines: LogInfo,
    pub internals: LogInfo,
}
#[derive(Debug, Clone)]
pub struct LogInfo {
    pub file_info: Option<LogFile>,
    pub level: LevelFilter,
}
#[derive(Debug, Clone)]
pub struct LogFile {
    pub directory: String,
    pub name: String,
}

impl Default for Logger {
    fn default() -> Self {
        let logger = Logger {
            internals: LogInfo {
                file_info: None,
                level: LevelFilter::Error,
            },
            pipelines: LogInfo {
                file_info: None,
                level: LevelFilter::Error,
            },
        };
        logger
    }
}

impl Logger {
    pub fn set_internal_level(&mut self, level: &LevelFilter) -> Result<Self> {
        self.internals.level = level.to_owned();
        Ok(self.to_owned())
    }
    pub fn set_level(&mut self, level: &LevelFilter) -> Result<Self> {
        self.pipelines.level = level.to_owned();
        Ok(self.to_owned())
    }

    /**
    Create a log file with given uuid to log pipeline with:

    let json = serde_json::to_string(&pipeline).unwrap();
    error!(target: "pipelines_to_file","{}", json);

    */
    pub fn set_file(&mut self, uuid: &Uuid) -> Result<Self> {
        self.pipelines.file_info = Some(LogFile {
            name: uuid.to_string(),
            ..self.pipelines.file_info.clone().unwrap()
        });
        Ok(self.to_owned())
    }

    /**
     * Delete logs directories
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
impl Logger {
    pub fn to_file(&mut self) -> Self {
        let directory = format!(
            "{}/.pipelight/logs",
            &env::current_dir().unwrap().to_str().unwrap()
        );

        let logger = Logger {
            internals: LogInfo {
                file_info: None,
                // Uncomment to log internals to file
                //
                // Some(LogFile {
                // name: "_unlinked".to_owned(),
                // directory: format!(
                // "{}/.pipelight/_internals/logs",
                // &env::current_dir().unwrap().to_str().unwrap()
                // ),
                // }),
                //
                ..self.internals.clone()
            },
            pipelines: LogInfo {
                file_info: Some(LogFile {
                    name: "_unlinked".to_owned(),
                    directory,
                }),
                ..self.pipelines.clone()
            },
        };

        *self = logger;
        self.to_owned()
    }
    pub fn to_stdout(&mut self) -> Self {
        let logger = Logger {
            internals: LogInfo {
                file_info: None,
                ..self.internals.clone()
            },
            pipelines: LogInfo {
                file_info: None,
                ..self.pipelines.clone()
            },
        };
        *self = logger;
        self.to_owned()
    }
}
