// Exec subprocess
use crate::types::{Config, Path, PipelineLog};
use log::{debug, error, info, trace, warn};
use std::env;
use std::error::Error;
use std::io::Write;
use std::process::{Command, Stdio};

/// Execute in same subprocess
pub fn exec_attach(shell: String, command: String) -> Result<String, String> {
    let child = Command::new(shell)
        // .arg("-n")
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
        trace!("{}", stdout);
        Ok(stdout)
    } else {
        let stderr = String::from_utf8(output.stderr).unwrap().to_owned();
        error!("{}", stderr);
        Err(stderr)
    }
}

/// Return user session shell
pub fn get_shell() -> String {
    let shell = env::var("SHELL").expect("Could'nt retrieve user session shell");
    trace!("shell set to {}", shell);
    return shell;
}
pub fn exec(command: String) -> Result<String, Box<dyn Error>> {
    let user_shell = get_shell();
    let output = exec_attach(user_shell, command);
    let res = match output {
        Ok(output) => {
            return Ok(output);
        }
        Err(e) => {
            error!("{}", e);
            return Err(Box::from(e));
        }
    };
}
pub fn shell(command: String) -> Result<String, String> {
    let user_shell = get_shell();
    let output = exec_attach(user_shell, command.clone());
    match output {
        Ok(output) => {
            return Ok(output);
        }
        Err(e) => {
            error!("{}", e);
            return Err(e);
        }
    };
}
