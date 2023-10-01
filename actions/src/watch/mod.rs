// Test
mod test;
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
use miette::{IntoDiagnostic, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // 
    let ignore = Ignore::new(".pipelight_ignore").await?;

    let mut init = InitConfig::default();
    init.on_error(PrintDebug(std::io::stderr()));

    let mut runtime = RuntimeConfig::default();
    runtime.filterer(Arc::new(ignore));

    let we = Watchexec::new(init, runtime.clone())?;
    let w = we.clone();

    let c = runtime.clone();

    we.reconfigure(runtime);
    we.main().await.into_diagnostic()?;
    Ok(())
}
