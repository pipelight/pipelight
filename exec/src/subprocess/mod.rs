// sys
use std::process;
use std::process::{Command, Stdio};

use subprocess::{Exec, NullFile, Popen, PopenConfig, Redirection};
use sysinfo::{PidExt, ProcessExt, System, SystemExt};

// Types
use super::types::{Process, State, Status};
// File manipulation
use std::fs::{remove_file, File};
use std::io::BufReader;
use std::io::Read;

// Error Handling
use miette::{IntoDiagnostic, Result};
use uuid::Uuid;

// Tests
mod test;

/// Convert popen files to strings
impl Process {
    pub fn clean(&self) -> Result<()> {
        let stdout_path = format!("{}/{}_stdout", self.os.directory, self.uuid);
        let stderr_path = format!("{}/{}_stderr", self.os.directory, self.uuid);
        remove_file(stdout_path).into_diagnostic()?;
        remove_file(stderr_path).into_diagnostic()?;
        Ok(())
    }
    pub fn read(&mut self) -> Result<()> {
        // path definition
        let stdout_path = format!("{}/{}_stdout", self.os.directory, self.uuid);
        let stderr_path = format!("{}/{}_stderr", self.os.directory, self.uuid);

        let f = File::open(stdout_path).into_diagnostic()?;
        let mut buf_reader = BufReader::new(f);
        let mut stdout = String::new();
        buf_reader.read_to_string(&mut stdout).into_diagnostic()?;

        // stderr
        let f = File::open(stderr_path).into_diagnostic()?;
        let mut buf_reader = BufReader::new(f);
        let mut stderr = String::new();
        buf_reader.read_to_string(&mut stderr).into_diagnostic()?;

        let output = State {
            status: self.state.status.clone(),
            stdin: self.state.stdin.clone(),
            stdout: Some(stdout),
            stderr: Some(stderr),
        };
        self.state = output;
        Ok(())
    }
    /// Execute in same process
    pub fn simple(&mut self) -> Result<Self> {
        let child = Command::new(self.os.shell.clone())
            .arg("-c")
            .arg(self.state.stdin.clone().unwrap())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .into_diagnostic()?;

        let output = child.wait_with_output().into_diagnostic()?;
        let output = State {
            stdin: self.state.stdin.clone(),
            ..State::from(&output)
        };

        self.state = output;
        Ok(self.to_owned())
    }
    pub fn run(&mut self) -> Result<()> {
        let stdout = format!("{}/{}_stdout", self.os.directory, self.uuid);
        let stderr = format!("{}/{}_stderr", self.os.directory, self.uuid);

        let child = Command::new(self.os.shell.clone())
            .arg("-c")
            .arg(&self.state.stdin.clone().unwrap())
            .stdin(Stdio::null())
            .stdout(File::create(stdout).into_diagnostic()?)
            .stderr(File::create(stderr).into_diagnostic()?)
            .spawn()
            .into_diagnostic()?;

        let output = child.wait_with_output().into_diagnostic()?;

        self.state.status = State::from(&output).status;
        self.read();
        self.clean();

        println!("{:#?}", output);
        Ok(())
    }

    /// Execute in detached child subprocess
    pub fn detached(&mut self) -> Result<Self> {
        Command::new(self.os.shell.clone())
            .arg("-c")
            .arg(self.state.stdin.clone().unwrap())
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("Failed to spawn subprocess");

        Ok(self.to_owned())
    }
}
