// Test
mod test;
// Detach
// use crate::utils::should_detach;
// Globals
use std::sync::Arc;
// Watchexec
use watchexec::{
    action::{Action, Outcome},
    config::{InitConfig, RuntimeConfig},
    handler::{Handler as _, PrintDebug},
    Watchexec,
};
use utils::files::Ignore;
// Error handling
use miette::{IntoDiagnostic, Result, Diagnostic};
use thiserror::Error; 

mod build;
pub use build::*;
mod is;


impl Watcher {
pub fn launch() -> Result<()> {
    match should_detach(None)? {
        false => start()?,
        true => {}
    };
    Ok(())
}

pub fn kill() -> Result<()> {
    Watcher::kill_homologous()?;
    Ok(())
}
}

/**
Build and launch the custom watcher
*/
#[tokio::main]
pub async fn start() -> Result<()> {
    // Kill already running watcher
    Watcher::kill_homologous()?;
    let (we, runtime) = build()?;
    we.main().await.into_diagnostic()?;
    Ok(())
}

