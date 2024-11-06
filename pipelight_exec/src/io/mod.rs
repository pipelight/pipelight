use serde::{Deserialize, Serialize};
use uuid::Uuid;
// Globals
use crate::globals::OUTDIR;
// File manipulation
use std::fs::{remove_dir_all, File};
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
    pub uuid: Uuid,
    pub stdin: Option<String>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}

impl Io {
    /**
    Delete the files associated to the Io struct.
    */
    pub fn clean(&self) -> Result<(), std::io::Error> {
        let path = format!("{}/{}", *OUTDIR.lock().unwrap(), self.uuid);
        // Guard
        let path = Path::new(&path);
        if path.exists() && path.is_file() {
            remove_dir_all(path)?;
        }
        Ok(())
    }
    /**
     * Read the process stdout and stderr and stores it in the struct field
     *
     *
     * ```rust,ignore
     * # use pipelight_exec::Process;
     * # use miette::{Report, IntoDiagnostic};
     *
     * let mut p = Process::new().stdin("echo stuff").fs();
     * println!("{:?}", p.io.stdout); // None
     * p.io.read().into_diagnostic()?;
     * println!("{:?}", p.io.stdout); // Some("stuff\n")
     *
     * # Ok::<(), Report>(())
     * ```
     *  
     */
    pub fn read(&mut self) -> Result<(), std::io::Error> {
        // path definition
        let stdout_path = format!("{}/{}/1", *OUTDIR.lock().unwrap(), self.uuid);
        let stderr_path = format!("{}/{}/2", *OUTDIR.lock().unwrap(), self.uuid);

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
            uuid: Uuid::new_v4(),
            stdout,
            stderr,
        }
    }
}
