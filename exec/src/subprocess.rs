use super::types::StrOutput;
use std::process::{Command, Stdio};

// Error Handling
use miette::{Diagnostic, Error, IntoDiagnostic, NamedSource, Report, Result, SourceSpan};
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
