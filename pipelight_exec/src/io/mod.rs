use serde::{Deserialize, Serialize};
use uuid::Uuid;
// Globals
use crate::globals::OUTDIR;
// File manipulation
use std::fs::{remove_file, File};
use std::io::BufReader;
use std::io::Read;
use std::path::Path;
use std::process::Output;

// Error Handling
use log::info;
use miette::{IntoDiagnostic, Result};
use pipelight_error::PipelightError;

/**
* A struct that stores the procees standards input and outputs into human readable strings.
*/
#[derive(Default, Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Io {
    pub uuid: Option<Uuid>,
    pub stdin: Option<String>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}

impl Io {
    /**
    Delete the files associated to the Io struct.
    */
    pub fn clean(&self) -> Result<(), std::io::Error> {
        // path definition
        let stdout_path = format!("{}/{}_stdout", *OUTDIR.lock().unwrap(), self.uuid.unwrap());
        let stderr_path = format!("{}/{}_stderr", *OUTDIR.lock().unwrap(), self.uuid.unwrap());
        // Guard
        let stdout = Path::new(&stdout_path);
        if stdout.exists() && stdout.is_file() {
            remove_file(stdout)?;
        }
        // Guard
        let stderr = Path::new(&stderr_path);
        if stderr.exists() && stderr.is_file() {
            remove_file(stderr)?;
        }
        Ok(())
    }
    /**
    Read the files associated to the Io struct and hydrate
    the Io stdout and stderr fields.
    */
    pub fn read(&mut self) -> Result<(), std::io::Error> {
        // path definition
        let stdout_path = format!("{}/{}_stdout", *OUTDIR.lock().unwrap(), self.uuid.unwrap());
        let stderr_path = format!("{}/{}_stderr", *OUTDIR.lock().unwrap(), self.uuid.unwrap());

        // stdout
        info!("read subprocess stdout from tmp file at {}", stdout_path);
        let f = File::open(stdout_path)?;
        let mut buf_reader = BufReader::new(f);
        let mut stdout = String::new();
        buf_reader.read_to_string(&mut stdout)?;

        // stderr
        info!("Read subprocess stderr from tmp file at {}", stderr_path);
        let f = File::open(stderr_path)?;
        let mut buf_reader = BufReader::new(f);
        let mut stderr = String::new();
        buf_reader.read_to_string(&mut stderr)?;

        *self = Io {
            stdin: self.stdin.to_owned(),
            stdout: Some(stdout),
            stderr: Some(stderr),
            ..*self
        };
        Ok(())
    }
}
/**
* Convert a standart process (std::process) outputs into an Io struct.
* The output buffers are converted into human readable strings.
*/
impl From<&Output> for Io {
    fn from(output: &Output) -> Io {
        let stdout_str = String::from_utf8(output.stdout.to_owned()).unwrap();
        // .strip_suffix("\r\n")
        // .unwrap()
        let stderr_str = String::from_utf8(output.stderr.to_owned()).unwrap();
        // .strip_suffix("\r\n")
        // .unwrap()

        let mut stdout = None;
        if !stdout_str.is_empty() {
            stdout = Some(stdout_str);
        }
        let mut stderr = None;
        if !stderr_str.is_empty() {
            stderr = Some(stderr_str);
        }
        Io {
            stdin: None,
            uuid: None,
            stdout,
            stderr,
        }
    }
}
