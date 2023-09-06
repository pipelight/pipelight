// Error Handling
use miette::{Error, IntoDiagnostic, Result};

// Read file
use std::fs;
use std::path::Path;
use utils::files::read_last_line;

// Traits
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Logs;

impl Logs {
    pub fn read(directory_path: &str) -> Result<Vec<String>> {
        let mut logs: Vec<String> = vec![];
        let message = "No logs to display.";

        // Directory Safe-guard
        if !Path::new(directory_path).exists() {
            return Err(Error::msg(message));
        }
        // Files Safe-guard
        let entries = fs::read_dir(directory_path).into_diagnostic()?;
        for entry in entries {
            let entry = entry.into_diagnostic()?;
            if entry.file_type().into_diagnostic()?.is_file() {
                let res = read_last_line(&entry.path());
                match res {
                    Ok(json) => {
                        logs.push(json);
                    }
                    Err(_err) => {
                        // warn!("Striping corrupted log")
                    }
                }
            }
        }
        Ok(logs)
    }
}
