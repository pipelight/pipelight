// sys
use std::process;
use std::process::{Command, Stdio};

use subprocess::{Exec, NullFile, Popen, PopenConfig, Redirection};
use sysinfo::{PidExt, ProcessExt, System, SystemExt};

// Types
use super::types::{Process, State};

// Error Handling
use miette::{IntoDiagnostic, Result};

/// Execute in same process
impl Process {
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
    pub fn create(&self) -> Result<(Self, u32)> {
        let mut child = Exec::shell(self.env.shell.clone())
            .arg("-c")
            .arg(self.state.stdin.clone().unwrap())
            .stdin(NullFile)
            .stdout(Redirection::Pipe)
            .stderr(Redirection::Pipe)
            .popen()
            .into_diagnostic()?;

        let pid = child.pid().unwrap();

        let output = child.communicate(None).into_diagnostic()?;

        Ok((self.to_owned(), pid))
    }

    /// Execute in detached child subprocess
    pub fn detached(&self) -> Result<Self> {
        // retrieve PGID and link every subprocess to it
        let current_pid = process::id();
        let mut sys = System::new_all();
        sys.refresh_all();
        let current_process = sys.process(PidExt::from_u32(current_pid)).unwrap();
        let pgid = current_process.group_id().unwrap();
        Exec::shell(self.env.shell.clone())
            // .setpgid(*pgid)
            .arg("-c")
            .arg(self.state.stdin.clone().unwrap())
            .stdin(NullFile)
            .stdout(Redirection::Pipe)
            .stderr(Redirection::Pipe)
            .popen()
            .into_diagnostic()?;

        Ok(self.to_owned())
    }
}
