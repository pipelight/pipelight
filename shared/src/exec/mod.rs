// Exec subprocess

use crate::types::{Config, Path};
use log::{debug, error, info, trace, warn};
use std::env;
use std::error::Error;
use std::io::Write;
use std::process::{Command, Stdio};
use subprocess::{Exec, Popen, PopenConfig, Redirection};

/// Execute in same subprocess
pub fn exec_attach(command: String) -> Result<String, Box<dyn Error>> {
    let escaped_command = format!("{:?}", command);
    // let mut v: Vec<&str> = command.split(' ').collect();
    // let cmd: String = v.remove(0).to_owned();
    // let args = v;

    // let shell = env::var("SHELL").unwrap();
    // let mut child = Command::new(shell)
    //     .stdin(Stdio::piped())
    //     .stderr(Stdio::piped())
    //     .stdout(Stdio::piped())
    //     // .spawn()?
    //     .spawn()
    //     .expect("failed to execute child");
    //
    // child
    //     .stdin
    //     .as_mut()
    //     .ok_or("Child process stdin has not been captured!")?
    //     .write_all(command.as_bytes())?;

    let child = Command::new("zsh")
        .arg("-i")
        .arg("-c")
        .arg(format!("\"{}\"", escaped_command))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to execute child");

    let output = child
        .wait_with_output()
        .expect("Failed to wait on child process");
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout)?;
        trace!("{}", stdout);
        Ok(stdout)
    } else {
        let stderr = String::from_utf8(output.stderr)?;
        error!("{}", stderr);
        Err(Box::from(stderr))
    }
}

/// Detect shell
pub fn get_shell() -> String {
    let shell: Vec<(String, String)> = env::vars().collect();
    let cur_path = env::var("PATH").unwrap();
    println!("{:#?}", shell);
    // let cmd = "echo $0".to_owned();
    // let shell = exec_attach(cmd).expect("Couldn't reach shell");
    // return shell;
    return "zsh".to_owned();
}
pub fn shell(command: String) -> String {
    let out = Exec::cmd("zsh")
        .arg("-i")
        .arg("-c")
        .arg("neo")
        .stdout(Redirection::Pipe)
        .capture()
        .expect("Couldn't exec")
        .stdout_str();
    println!("{}", out);
    let shell = get_shell();
    let res = exec_attach(format!("\"{}\"", command)).expect("Couldn't exec command");
    return res;
}
