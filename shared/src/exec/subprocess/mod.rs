use log::{debug, error, info, trace, warn};
use std::env;
use std::error::Error;
use std::process::{Command, Stdio};

pub fn exec_attached<'a>(command: &str) -> Result<String, Box<dyn Error>> {
    let user_shell = get_shell();
    let output = subprocess_attached(&user_shell, command);
    let res = match output {
        Ok(output) => {
            return Ok(output.to_owned());
        }
        Err(e) => {
            error!("command: {}\n output: {}", command, e);
            return Err(Box::from(e));
        }
    };
}
pub fn exec_detached(command: &str) -> Result<(), Box<dyn Error>> {
    let user_shell = get_shell();
    let output = subprocess_detached(&user_shell, command);
    Ok(())
}

/// Execute in same subprocess
pub fn subprocess_attached<'a>(shell: &str, command: &str) -> Result<String, String> {
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
fn subprocess_detached(shell: &str, command: &str) -> Result<(), Box<dyn Error>> {
    let child = Command::new(shell)
        .arg("-c")
        .arg(command)
        .spawn()
        .expect("Failed to spawn subprocess");
    Ok(())
}
/// Return user session shell
pub fn get_shell<'a>() -> String {
    let default_shell = "sh".to_owned();
    let shell_result = env::var("SHELL");
    let shell = match shell_result {
        Ok(res) => {
            return res.to_owned();
        }
        Err(e) => {
            return default_shell.to_owned();
        }
    };
}
