// Call ts-node on mjs/ts files

use crate::types::Config;
use log::{debug, error, info, trace, warn};
use project_root::get_project_root;
use std::default::Default;
use std::error::Error;
use subprocess::{Exec, Popen, PopenConfig, PopenError};

/// Return the config from .ts file inside the working dir.
pub fn load_config() -> Result<Config, Box<dyn Error>> {
    let root = get_project_root()?;
    let command = "ts-node";
    let entrypoint = "public/load_config.ts";
    let data = { Exec::shell(format!("{} {:#?}/{}", command, root, entrypoint)) }
        .capture()?
        .stdout_str();

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
