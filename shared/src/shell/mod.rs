// Call ts-node on mjs/ts files

use crate::types::Config;
use log::{debug, error, info, trace, warn};
use project_root::get_project_root;
use std::any::Any;
use std::collections::HashMap;
use std::default::Default;
use std::error::Error;
use subprocess::{Exec, Popen, PopenConfig, PopenError, Redirection};

struct Path {
    folder: String,
    file: String,
}
/// Execute in an attached subprocess
pub fn exec_attach(command: String) -> Result<String, Box<dyn Error>> {
    println!("{}", command);
    let stdout = { Exec::shell(format!("{}", command)) }
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .capture()?
        .stdout_str();

    println!("{}", stdout);
    Ok(stdout)
}
/// Execute in a detached subprocess
// pub fn exec_detach(command: String) -> Result<String, Box<dyn Error>> {}

/// Return the config from .ts file inside the working dir.
pub fn load_config() -> Result<Config, Box<dyn Error>> {
    let executable = "ts-node";
    let path = Path {
        folder: get_project_root()?.to_str().unwrap().to_owned(),
        file: "typescript/main.ts".into(),
    };
    let command = format!("{} {}/{}", executable, path.folder, path.file);

    exec_attach("ls".to_owned())?;

    let data = exec_attach(command)?;
    let config_result = serde_json::from_str(&data);
    let config = match config_result {
        Ok(res) => {
            println!("{:#?}", res);
            return Ok(res);
        }
        Err(e) => {
            println!("{:#?}", e);
            return Err(Box::from(e));
        }
    };
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
