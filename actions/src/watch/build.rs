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

// Watchexec - Bug fix Struct
#[derive(Debug, Error, Diagnostic)]
#[error("stub")] 
struct MietteStub;


pub async fn build() -> Result<(Arc<Watchexec>, RuntimeConfig)> {
  // Default config
  let mut init = InitConfig::default();
  init.on_error(PrintDebug(std::io::stderr()));

  // Set Filter
  // Parse ignore file into watchexec filter
  let file = ".pipelight_ignore";
  let ignore = Ignore::new(file).await?;
  let mut runtime = RuntimeConfig::default();
  runtime.filterer(Arc::new(ignore));

  // Create WE instance
  let we = Watchexec::new(init, runtime.clone())?;
  let w = we.clone();

  let r = runtime.clone();
  runtime.on_action(move |action: Action| {    
    let mut r = r.clone();
    let w = w.clone();

    // Self reconfigure when the IgnoreFile changes.
    async move {  
      for event in action.events.iter() {   
        if event.paths().any(|(p, _)| p.ends_with(file)) { 
          let ignore = Ignore::new(file).await.unwrap();
          // Set Filter
          r.filterer(Arc::new(ignore));
          w.reconfigure(r.clone()).unwrap();
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

    // Execute the pipelines
    
    
  });
  Ok((we,runtime))
}

