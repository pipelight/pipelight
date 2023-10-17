// Test
mod test;
// Detach
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

#[derive(Debug)]
pub struct Watcher {
    watchexec: Option<Watchexec>,
    runtime: Option<RuntimeConfig>,
    builded: bool,
}

impl Watcher {

pub fn kill() -> Result<()> {
    Watcher::kill_homologous()?;
    Ok(())
}
/**
Build and launch the custom watcher
*/
#[tokio::main]
pub async fn start() -> Result<()> {
    // Kill already running watcher
    Watcher::kill_homologous()?;
    let (we, runtime) = build()?;

    we.reconfigure(runtime)?;
    we.main().await.into_diagnostic()?;
    Ok(())
}
}


