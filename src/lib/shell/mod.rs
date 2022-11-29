// Call ts-node on mjs/ts files

use crate::types::Config;
use log::{debug, error, info, trace, warn};
use project_root::get_project_root;
use std::collections::HashMap;
use std::default::Default;
use std::error::Error;
use subprocess::{Exec, Popen, PopenConfig, PopenError};

struct Path {
    folder: String,
    file: String,
}
/// Execute in an attached subprocess
pub fn exec_attach(command: String) -> Result<String, Box<dyn Error>> {
    let stdout = { Exec::shell(format!("{}", command)) }
        .capture()?
        .stdout_str();
    debug!("{}", stdout);
    Ok(stdout)
}
/// Execute in a detached subprocess
pub fn exec_detach(command: String) -> Result<String, Box<dyn Error>> {
    let stdout = { Exec::shell(format!("{}", command)) }
        .capture()?
        .stdout_str();
    debug!("{}", stdout);
    Ok(stdout)
}

/// Return the config from .ts file inside the working dir.
pub fn load_config() -> Result<Config, Box<dyn Error>> {
    let executable = "ts-node";
    let path = Path {
        folder: format!("{:#?}", get_project_root()?),
        file: "typescript/main.ts".into(),
    };
    let command = format!("{} {}/{}", executable, path.folder, path.file);
    let data = exec_attach(command)?;
    let res = serde_json::from_str(&data)?;

    debug!("{:#?}", res);

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::type_of;
    use std::env;
    use std::path::Path;
    #[test]
    fn internal() {
        let root = Path::new("./test");
        assert!(env::set_current_dir(&root).is_ok());
        println!(
            "Successfully changed working directory to {:#?}!",
            root.display()
        );
        // set cwd to test in other folder
        let res = load_config();
    }
}
