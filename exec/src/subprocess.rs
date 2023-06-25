use super::types::StrOutput;
use std::os::unix::process::CommandExt;

// sys
use std::process;
use std::process::{Command, Stdio};
use sysinfo::{Pid, PidExt, ProcessExt, System, SystemExt};

// Error Handling
use miette::{Diagnostic, Error, IntoDiagnostic, NamedSource, Report, Result, SourceSpan};
use thiserror::Error;

/// Execute in same process
pub fn simple<'a>(shell: &str, command: &str) -> Result<StrOutput> {
    // retrieve PGID
    let current_pid = process::id();
    let mut sys = System::new_all();
    sys.refresh_all();
    let current_process = sys.process(PidExt::from_u32(current_pid)).unwrap();
    let pgid = current_process.group_id().unwrap();

    let child = Command::new(shell)
        .gid(*pgid)
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .into_diagnostic()?;

    let output = child.wait_with_output().into_diagnostic()?;
    Ok(StrOutput::from(&output))
}

/// Execute in detached child subprocess
pub fn detached(shell: &str, command: &str) -> Result<()> {
    // retrieve PGID
    // let current_pid = process::id();
    // let mut sys = System::new_all();
    // sys.refresh_all();
    // let current_process = sys.process(PidExt::from_u32(current_pid)).unwrap();
    // let pgid = current_process.group_id().unwrap();

    Command::new(shell)
        // .gid(*pgid)
        .arg("-c")
        .arg(command)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to spawn subprocess");

    Ok(())
}
