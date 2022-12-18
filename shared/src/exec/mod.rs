// Exec subprocess
pub mod pipeline;
use crate::types::logs::PipelineLog;
use log::{debug, error, info, trace, warn};
use std::env;
use std::error::Error;
use std::io::Write;
use std::process::{Command, Stdio};

/// Execute in same subprocess
pub fn subprocess_attached(shell: String, command: String) -> Result<String, String> {
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

    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).unwrap().to_owned();
        Ok(stdout)
    } else {
        let stderr = String::from_utf8(output.stderr).unwrap().to_owned();
        Err(stderr)
    }
}
pub fn subprocess_detached(shell: String, command: String) -> Result<(), Box<dyn Error>> {
    let child = Command::new(shell)
        .arg("-c")
        .arg(command)
        .spawn()
        .expect("Failed to spawn subprocess");
    Ok(())
}

/// Return user session shell
pub fn get_shell() -> Result<String, String> {
    let default_shell = "sh".to_owned();

    let shell_result = env::var("SHELL");
    let shell = match shell_result {
        Ok(res) => {
            return Ok(res);
        }
        Err(e) => {
            return Err(default_shell);
        }
    };
}
pub fn exec(command: String) -> Result<String, Box<dyn Error>> {
    let user_shell = get_shell()?;
    let output = subprocess_attached(user_shell, command.clone());
    let res = match output {
        Ok(output) => {
            return Ok(output);
        }
        Err(e) => {
            error!("command: {}\n output: {}", command, e);
            return Err(Box::from(e));
        }
    };
}
pub fn exec_detached(command: String) -> Result<(), Box<dyn Error>> {
    let user_shell = get_shell()?;
    subprocess_detached(user_shell, command.clone())?;
    Ok(())
}
pub fn shell(command: String) -> Result<String, String> {
    let user_shell = get_shell()?;
    let output = subprocess_attached(user_shell, command.clone());
    match output {
        Ok(output) => {
            return Ok(output);
        }
        Err(e) => {
            return Err(e);
        }
    };
}
