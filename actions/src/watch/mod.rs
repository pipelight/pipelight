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
use miette::{IntoDiagnostic, Result, Diagnostic};
use thiserror::Error; 


mod build;
pub use build::*;


#[tokio::main]
pub async fn launch() -> Result<()> {
  let (we, runtime) = build().await? ;

  // we.reconfigure(runtime);
  //
  we.main().await.into_diagnostic()?;

  Ok(())
}

