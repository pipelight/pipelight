// sys
use std::process;
use std::process::{Command, Stdio};

use subprocess::{Exec, NullFile, Popen, PopenConfig, Redirection};
use sysinfo::{PidExt, ProcessExt, System, SystemExt};

// Types
use super::types::{Process, State, Status};
use std::io::BufReader;
use std::io::Read;

// Error Handling
use miette::{IntoDiagnostic, Result};

/// Convert popen files to strings

/// Execute in same process
impl Process {
    fn popen_to_string(&mut self, popen: &Popen) -> Result<()> {
        println!("{:?}", popen);
        // stdout
        let mut buf_reader = BufReader::new(popen.stdout.as_ref().unwrap());
        let mut stdout = String::new();
        buf_reader.read_to_string(&mut stdout).into_diagnostic()?;

        // stderr
        let mut buf_reader = BufReader::new(popen.stderr.as_ref().unwrap());
        let mut stderr = String::new();
        buf_reader.read_to_string(&mut stderr).into_diagnostic()?;

        println!("{:?}", stdout);
        println!("{:?}", stderr);

        let output = State {
            stdin: self.state.stdin.clone(),
            stdout: Some(stdout),
            stderr: Some(stderr),
            ..State::default()
        };
        self.state = output;
        Ok(())
    }
    pub fn simple(&mut self) -> Result<Self> {
        let child = Command::new(self.env.shell.clone())
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
    pub fn create(&self) -> Result<(Self, Popen, u32)> {
        println!("{:?}", self.state.stdin);

        let mut popen = Exec::shell(self.env.shell.clone())
            .stdin(Redirection::Pipe)
            .stdout(Redirection::Pipe)
            .stderr(Redirection::Pipe)
            .popen()
            .into_diagnostic()?;

        let pid = popen.pid().unwrap();

        // println!("{:?}", popen);

        Ok((self.to_owned(), popen, pid))
    }
    pub fn run(&mut self, mut popen: Popen) -> Result<Self> {
        // let status = popen.wait().into_diagnostic()?;

        // println!("{:?}", status);

        let (stdout, stderr) = popen
            .communicate(Some(&self.state.stdin.as_ref().unwrap()))
            .into_diagnostic()?;

        let status = popen.poll();
        if status.is_some() {
            if status.unwrap().success() {
                self.state.status = Some(Status::Succeeded);
            } else {
                self.state.status = Some(Status::Failed);
            }
        }

        println!("{:?}", stdout);
        println!("{:?}", stderr);

        let state = State {
            status: self.state.status.clone(),
            stdin: self.state.stdin.clone(),
            stdout,
            stderr,
            ..State::default()
        };
        // self.popen_to_string(&popen);
        self.state = state;
        Ok(self.to_owned())
    }

    /// Execute in detached child subprocess
    pub fn detached(&mut self) -> Result<Self> {
        Command::new(self.env.shell.clone())
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
