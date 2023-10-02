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
  use miette::{IntoDiagnostic, Result, Diagnostic};
  use thiserror::Error; 

// ignore this! it's stuff to make the above code get checked by cargo doc tests! 
struct YourConfigFormat;
impl YourConfigFormat {
  async fn load_from_file(_: &str) -> std::result::Result<Self, MietteStub> {
    Ok(Self) 
  } 
  fn apply(&self, _: &mut RuntimeConfig) {} 
} 

#[derive(Debug, Error, Diagnostic)]
#[error("stub")] 
struct MietteStub;

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
  #[tokio::test]
  async fn set_reconfigure() -> Result<()> {
    // Teleport
    Portal::new()?.seed("test.pipelight").search()?.teleport()?;

    // Default config
    let mut init = InitConfig::default();
    init.on_error(PrintDebug(std::io::stderr()));

    // Set Filter
    // Parse ignore file into watchexec filter
    let file = ".pipelight_ignore";
    let ignore = Ignore::new(&file).await?;
    let mut runtime = RuntimeConfig::default();
    runtime.filterer(Arc::new(ignore));

    // Create WE instance
    let we = Watchexec::new(init, runtime.clone())?;
    let w = we.clone();

    /**
    Self reconfigure when ignore file changes
    */
    let r = runtime.clone();
    runtime.on_action(move |action: Action| {    
      let mut r = r.clone();
      let w = w.clone();
      async move {  
        for event in action.events.iter() {   
          if event.paths().any(|(p, _)| p.ends_with(file)) { 
            let ignore = Ignore::new(&file).await.unwrap();
            // Set Filter
            r.filterer(Arc::new(ignore));
            w.reconfigure(r.clone());
            break;
          }
        }
        action.outcome(Outcome::if_running( 
            Outcome::DoNothing,
            Outcome::both(Outcome::Clear, Outcome::Start),             
        ));
        Ok(())
        // (not normally required! ignore this when implementing)             
        as std::result::Result<_, MietteStub>
      }
    });
    we.reconfigure(runtime);

    // Generate the watcher.
    // Not that it is not awaited/started.
    we.main();
    Ok(())
  }
}
