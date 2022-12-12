// Call ts-node on mjs/ts files

use crate::types::{Config, Path};
use log::{debug, error, info, trace, warn};
use std::error::Error;
use std::process::{Command, Stdio};

/// Execute in same subprocess
pub fn exec_attach(command: String) -> Result<String, Box<dyn Error>> {
    debug!("{}", command);

    let mut v: Vec<&str> = command.split(' ').collect();
    let cmd: String = v.remove(0).to_owned();
    let args = v;

    let child = Command::new(cmd)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to execute child");

    let output = child.wait_with_output().expect("failed to wait on child");
    let stdout = String::from_utf8(output.stdout)?;

    trace!("{}", stdout);
    Ok(stdout)
}
