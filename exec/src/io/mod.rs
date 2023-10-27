// Structs
use super::types::Io;
// Globals
use crate::globals::OUTDIR;
// File manipulation
use std::fs::{remove_file, File};
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

// Error Handling
use log::info;
use miette::{IntoDiagnostic, Result};

impl Io {
    /**
    Delete the files associated to the Io struct.
    */
    pub fn clean(&self) -> Result<()> {
        // path definition
        let stdout_path = format!("{}/{}_stdout", *OUTDIR.lock().unwrap(), self.uuid.unwrap());
        let stderr_path = format!("{}/{}_stderr", *OUTDIR.lock().unwrap(), self.uuid.unwrap());
        // Guard
        let stdout = Path::new(&stdout_path);
        if stdout.exists() && stdout.is_file() {
            remove_file(stdout).into_diagnostic()?;
        }
        // Guard
        let stderr = Path::new(&stderr_path);
        if stderr.exists() && stderr.is_file() {
            remove_file(stderr).into_diagnostic()?;
        }
        Ok(())
    }
    /**
    Read the files associated to the Io struct and hydrate
    the Io stdout and stderr fields.
    */
    pub fn read(&mut self) -> Result<()> {
        // path definition
        let stdout_path = format!("{}/{}_stdout", *OUTDIR.lock().unwrap(), self.uuid.unwrap());
        let stderr_path = format!("{}/{}_stderr", *OUTDIR.lock().unwrap(), self.uuid.unwrap());

        // stdout
        info!("read subprocess stdout from tmp file at {}", stdout_path);
        let f = File::open(stdout_path).into_diagnostic()?;
        let mut buf_reader = BufReader::new(f);
        let mut stdout = String::new();
        buf_reader.read_to_string(&mut stdout).into_diagnostic()?;

        // stderr
        info!("Read subprocess stderr from tmp file at {}", stderr_path);
        let f = File::open(stderr_path).into_diagnostic()?;
        let mut buf_reader = BufReader::new(f);
        let mut stderr = String::new();
        buf_reader.read_to_string(&mut stderr).into_diagnostic()?;

        *self = Io {
            stdin: self.stdin.to_owned(),
            stdout: Some(stdout),
            stderr: Some(stderr),
            ..*self
        };
        Ok(())
    }
}
