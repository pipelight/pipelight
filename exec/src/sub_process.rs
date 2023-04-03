use super::types::StrOutput;
use std::os::unix::io::AsFd;

use crate::types::Status;
use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use subprocess::{Communicator, Exec, Redirection};
// Error Handling
use miette::{Diagnostic, Error, IntoDiagnostic, NamedSource, Report, Result, SourceSpan};
use std::io;
use thiserror::Error;

/// Execute in same process
pub fn simple<'a>(shell: &str, command: &str) -> Result<StrOutput> {
    let child = Command::new(shell)
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .into_diagnostic()?;
    let output = child.wait_with_output().into_diagnostic()?;
    Ok(StrOutput::from(&output))
}
// fn lines_stream(child: &mut Child) -> impl Stream<Item = String, Error = Error> + Send + 'static {
//     let stdout = child
//         .stdout()
//         .take()
//         .expect("child did not have a handle to stdout");
//     tokio::io::lines(BufReader::new(stdout))
//         .map_err(|e| Error::from(e))
//         .inspect(|line| println!("Line: {}", line))
// }

// pub fn simple_early(shell: &str, command: &str) -> Result<Communicator> {
pub fn simple_early(shell: &str, command: &str) -> Result<Child> {
    let mut child: Child = Command::new(shell)
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .into_diagnostic()?;
    Ok(child)
}

//     let stdout = child.stdout.as_mut().unwrap();
//     let stdout_reader = BufReader::new(stdout);
//     let stdout_lines = stdout_reader.lines();
//
//     let mut out = None;
//     for line in stdout_lines {
//         out = Some(format!("{:?}", line));
//     }
//
//     let stderr = child.stderr.as_mut().unwrap();
//     let stderr_reader = BufReader::new(stderr);
//     let stderr_lines = stderr_reader.lines();
//
//     let mut err = None;
//     for line in stderr_lines {
//         err = Some(format!("{:?}", line));
//     }
//
//     let ecode = child.wait().expect("failed to wait on child");
//
//     let status = match ecode.success() {
//         true => Status::Succeeded,
//         false => Status::Failed,
//     };
//
//     Ok(StrOutput {
//         stdout: out,
//         stderr: err,
//         status: Some(status),
//     })
// }

/// Execute in detached child subprocess
pub fn detached(shell: &str, command: &str) -> Result<()> {
    Command::new(shell)
        .arg("-c")
        .arg(command)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to spawn subprocess");
    Ok(())
}
