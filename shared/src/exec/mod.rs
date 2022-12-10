// Call ts-node on mjs/ts files

use crate::types::{Config, Path};
use log::{debug, error, info, trace, warn};
use std::error::Error;
use subprocess::{Exec, Popen, PopenConfig, PopenError, Redirection};

/// Execute in same subprocess
pub fn exec_attach(command: String) -> Result<String, Box<dyn Error>> {
    debug!("{}", command);
    let stdout = { Exec::shell(format!("{}", command)) }
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .capture()?
        .stdout_str();

    trace!("{}", stdout);
    Ok(stdout)
}
/// Execute in a detached subprocess
pub fn exec_detach(command: String) -> Result<String, Box<dyn Error>> {
    debug!("{}", command);
    let stdout = { Exec::shell(format!("{}", command)) }
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .capture()?
        .stdout_str();

    trace!("{}", stdout);
    Ok(stdout)
}
