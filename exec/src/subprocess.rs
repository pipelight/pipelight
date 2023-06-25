// sys
use std::os::unix::process::CommandExt;
use std::process;
use std::process::{Command, Stdio};
use sysinfo::{PidExt, ProcessExt, System, SystemExt};

// Types
use super::types::Process;

// Error Handling
use miette::{IntoDiagnostic, Result};

/// Execute in same process
impl Process {
    pub fn simple(&self) -> Result<Self> {
        // retrieve PGID and link every subprocess to
        let current_pid = process::id();
        let mut sys = System::new_all();
        sys.refresh_all();
        let current_process = sys.process(PidExt::from_u32(current_pid)).unwrap();
        let pgid = current_process.group_id().unwrap();

        let child = Command::new(self.env.shell.clone())
            .gid(*pgid)
            .arg("-c")
            .arg(self.state.stdin.clone().unwrap())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .into_diagnostic()?;

        let output = child.wait_with_output().into_diagnostic()?;
        Ok(self.to_owned())
    }

    /// Execute in detached child subprocess
    pub fn detached(&self) -> Result<()> {
        Command::new(self.env.shell.clone())
            .arg("-c")
            .arg(self.state.stdin.clone().unwrap())
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("Failed to spawn subprocess");

        Ok(())
    }
}
