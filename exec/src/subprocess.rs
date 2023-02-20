use super::types::StrOutput;
use std::error::Error;
use std::process::{Command, Stdio};

/// Execute in same process
pub fn simple<'a>(shell: &str, command: &str) -> Result<StrOutput, Box<dyn Error>> {
    let child = Command::new(shell)
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let output = child.wait_with_output()?;
    Ok(StrOutput::from(&output))
}

/// Execute in detached child subprocess
pub fn detached(shell: &str, command: &str) -> Result<(), Box<dyn Error>> {
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
