// sys
use std::process;
use std::process::{Command, Stdio};
// use subprocess::{Exec, NullFile, Popen, PopenConfig, Redirection};
// use sysinfo::{PidExt, ProcessExt, System, SystemExt};

// Types
use crate::types::{Io, Process, State, Status};
use utils::dates::Duration;

// File manipulation
use std::fs::{create_dir_all, remove_file, File};
use std::io::BufReader;
use std::io::Read;

// Error Handling
use log::info;
use miette::{IntoDiagnostic, Result};
use uuid::Uuid;

// Globals
use crate::globals::{OUTDIR, SHELL};

// Tests
mod test;

/// Convert popen files to strings

impl Process {
    /// Execute in same process
    /// Old name: simple
    pub fn run_piped(&mut self) -> Result<()> {
        let mut duration = Duration::default();
        let child = Command::new(&(*SHELL.lock().unwrap()))
            .arg("-c")
            .arg(&self.io.stdin.as_ref().unwrap())
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .into_diagnostic()?;

        // Hydrate struct
        duration.start();
        let output = child.wait_with_output().into_diagnostic()?;
        duration.stop();
        self.io = Io {
            uuid: self.io.uuid,
            stdin: self.io.stdin.to_owned(),
            ..Io::from(&output)
        };
        self.state = State {
            duration: Some(duration),
            status: Some(Status::from(&output)),
        };
        Ok(())
    }
    pub fn run_fs(&mut self) -> Result<()> {
        info!("Run detached subprocess");
        let mut duration = Duration::default();
        // path definition
        create_dir_all(&(*OUTDIR.lock().unwrap())).into_diagnostic()?;
        let stdout_path = format!("{}/{}_stdout", *OUTDIR.lock().unwrap(), self.uuid.unwrap());
        let stderr_path = format!("{}/{}_stderr", *OUTDIR.lock().unwrap(), self.uuid.unwrap());

        // Ensure internal log dir exists
        let child = Command::new(&(*SHELL.lock().unwrap()))
            .arg("-c")
            .arg(&self.io.stdin.as_ref().unwrap())
            .stdin(Stdio::null())
            .stdout(File::create(&stdout_path).into_diagnostic()?)
            .stderr(File::create(&stderr_path).into_diagnostic()?)
            .spawn()
            .into_diagnostic()?;

        // Hydrate struct
        duration.start();
        let output = child.wait_with_output().into_diagnostic()?;
        duration.stop();
        self.io.read()?;
        self.io.clean()?;
        self.state = State {
            duration: Some(duration),
            status: Some(Status::from(&output)),
        };
        Ok(())
    }

    /// Execute in detached child sub
    /// Old name: detached
    pub fn run_muted(&mut self) -> Result<()> {
        let mut duration = Duration::default();
        let child = Command::new(&(*SHELL.lock().unwrap()))
            .arg("-c")
            .arg(&self.io.stdin.as_ref().unwrap())
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .into_diagnostic()?;

        // Hydrate struct
        duration.start();
        let output = child.wait_with_output().into_diagnostic()?;
        duration.stop();

        self.io = Io {
            uuid: self.io.uuid,
            stdin: self.io.stdin.to_owned(),
            ..Io::from(&output)
        };
        self.state = State {
            duration: Some(duration),
            status: Some(Status::from(&output)),
        };
        Ok(())
    }
    /// Execute in detached child sub
    /// Old name: detached
    pub fn run_detached(&mut self) -> Result<()> {
        let mut duration = Duration::default();
        duration.start();
        Command::new(&(*SHELL.lock().unwrap()))
            .arg("-c")
            .arg(&self.io.stdin.as_ref().unwrap())
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .into_diagnostic()?;
        duration.stop();
        self.state = State {
            duration: Some(duration),
            status: Some(Status::Succeeded),
        };
        Ok(())
    }
}
