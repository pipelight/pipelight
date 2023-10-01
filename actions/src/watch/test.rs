#[cfg(test)]
mod watch {
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
  use utils::teleport::Portal;
  // Error handling
  use miette::{IntoDiagnostic, Result};

  #[tokio::test]
  async fn set_runtime() -> Result<()> {
    // Teleport
    Portal::new()?.seed("test.pipelight").search()?.teleport()?;

    // Parse ignore file into watchexec filter
    let ignore = Ignore::new("./.pipelight_ignore").await?;

    // Default config
    let mut init = InitConfig::default();
    init.on_error(PrintDebug(std::io::stderr()));

    // Set Filter
    let mut runtime = RuntimeConfig::default();
    runtime.filterer(Arc::new(ignore));

    Ok(())
  }
}
