// Process
use super::types::StrOutput;
use std::os::unix::process::CommandExt;
use std::process;
use std::process::{Command, Stdio};
use subprocess::{Exec, Redirection};

// sys
use sysinfo::{Pid, PidExt, ProcessExt, System, SystemExt};

use super::types::Status;

// Error Handling
use miette::{Diagnostic, Error, IntoDiagnostic, NamedSource, Report, Result, SourceSpan};
use thiserror::Error;

/// Execute in os
pub fn os(shell: &str, command: &str) -> Result<StrOutput> {
    // retrieve PGID
    let current_pid = process::id();
    let mut sys = System::new_all();
    sys.refresh_all();
    let current_process = sys.process(PidExt::from_u32(current_pid)).unwrap();
    let pgid = current_process.group_id().unwrap();

    let mut e: Vec<&str> = command.split(" ").collect();

    println!("{:?}", command);

    let child = Exec::cmd(e.remove(0))
        // let mut child = Exec::cmd("/usr/bin/zsh")
        // .gid(*pgid)
        // .arg("-c")
        .args(&e);
    // .stdin(Redirection::Pipe)
    // .stdout(Redirection::Pipe)
    // .stderr(Redirection::Pipe);
    println!("{:?}", child);
    // .popen()
    let output = child.capture().into_diagnostic()?;
    Ok(StrOutput::from(&output))
}

/// Execute in same process
pub fn simple(shell: &str, command: &str) -> Result<StrOutput> {
    // retrieve PGID
    let current_pid = process::id();
    let mut sys = System::new_all();
    sys.refresh_all();
    let current_process = sys.process(PidExt::from_u32(current_pid)).unwrap();
    let pgid = current_process.group_id().unwrap();

    // let e = command.split(" ");

    println!("{:?}", command);

    let mut child = Exec::shell(command)
        // let mut child = Exec::cmd("/usr/bin/zsh")
        // .gid(*pgid)
        // .arg("-c")
        // .arg(command)
        // .stdin(Redirection::Pipe)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Pipe);
    println!("{:?}", child);
    // .popen()
    let output = child.capture().into_diagnostic()?;

    // let exit = child.wait().into_diagnostic()?;
    // let child = Command::new("/usr/bin/zsh")
    //     .gid(*pgid)
    //     .arg("-c")
    //     .arg(command)
    //     .stdout(Stdio::piped())
    //     .stderr(Stdio::piped())
    //     .spawn()
    //     .into_diagnostic()?;

    // let output = child.wait_with_output().into_diagnostic()?;
    // child.capture();
    // let (out, err) = child.communicate(None).into_diagnostic()?;
    // let exit = child.wait().into_diagnostic()?;

    // println!("{:?}", capture);

    // println!("{:?}", out.clone());
    // println!("{:?}", err.clone());
    // println!("{:?}", exit.clone());

    Ok(StrOutput::from(&output))
}

/// Execute in detached child subprocess
pub fn detached(shell: &str, command: &str) -> Result<()> {
    // retrieve PGID
    let current_pid = process::id();
    let mut sys = System::new_all();
    sys.refresh_all();
    let current_process = sys.process(PidExt::from_u32(current_pid)).unwrap();
    let pgid = current_process.group_id().unwrap();

    Command::new(shell)
        .gid(*pgid)
        .arg("-c")
        .arg(command)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to spawn subprocess");

    Ok(())
}
