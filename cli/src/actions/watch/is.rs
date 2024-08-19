// Struct
use crate::actions::watch::Watcher;
// Env
use std::env;
// Process finder
use pipelight_exec::Finder;
// Error handling
use miette::{Error, IntoDiagnostic, Result};

impl Watcher {
    /**
    Return process info of any instance of pipelight watch
    that is already running on the current working directory.
    */
    pub fn find_any() -> Result<Finder> {
        let finder = Finder::new()
            .cwd(env::current_dir().into_diagnostic()?.to_str().unwrap())
            .seed("pipelight")
            .seed("watch")
            .search()?;
        Ok(finder)
    }
    /**
    Return process info of any instance of pipelight watch
    that is already running on the current working directory.
    */
    pub fn find_all() -> Result<Finder> {
        let finder = Finder::new()
            .root(env::current_dir().into_diagnostic()?.to_str().unwrap())
            .seed("pipelight")
            .seed("watch")
            .search()?;
        Ok(finder)
    }

    /**
    Check if an instance of 'pipelight watch' is already
    watching the current working directory.
    */
    pub fn has_homologous_already_running() -> Result<()> {
        let finder = Watcher::find_any()?;

        // Search homologous
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
        Finder::new()
            .cwd(env::current_dir().into_diagnostic()?.to_str().unwrap())
            .seed("pipelight")
            .seed("watch")
            .search()?
            .kill()?;
        Ok(())
    }
}
