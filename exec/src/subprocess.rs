use super::types::StrOutput;
use log::{debug, error, info, trace, warn};
use std::error::Error;
use std::process::{Command, Output, Stdio};

/// Execute in same subprocess
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

/// Execute in same subprocess
pub fn attached<'a>(shell: &str, command: &str) -> Result<String, String> {
    let child = Command::new(shell)
        // Intercative session, loads user variables like alias and profile
        // .arg("-i")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn subprocess");

    let output = child
        .wait_with_output()
        .expect("Failed to wait on child process");

    let output = StrOutput::from(&output);
    if output.status {
        Ok(output.stdout.unwrap())
    } else {
        Err(output.stderr.unwrap())
    }
}

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
