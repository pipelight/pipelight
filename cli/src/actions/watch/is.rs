// Struct
use crate::actions::watch::Watcher;
// Env
use std::env;
// Process finder
use exec::processes::Finder;
// Error handling
use miette::{Error, IntoDiagnostic, Result};

/**
Check if an instance of pipelight watch is already
watching the current working directory.
*/
impl Watcher {
    pub fn has_homologous_already_running() -> Result<()> {
        // Search homologous
        let finder = Finder::new()
            .cwd(env::current_dir().into_diagnostic()?.to_str().unwrap())
            .seed("pipelight")
            .seed("watch")
            .search()?;

        if finder.matches.is_some() {
            let message = "a watcher is already running on this project";
            //  let hint = "no need to re run another watcher";
            Err(Error::msg(message))
        } else {
            Ok(())
        }
    }
    pub fn kill_homologous() -> Result<()> {
        // Search homologous
        let finder = Finder::new()
            .cwd(env::current_dir().into_diagnostic()?.to_str().unwrap())
            .seed("pipelight")
            .seed("watch")
            .search()?
            .kill()?;
        Ok(())
    }
}
