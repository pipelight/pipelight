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

#[tokio::main]
async fn main() -> Result<()> {

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

  // Self reconfigure when ignore file changes
  let r = runtime.clone();
  runtime.on_action(move |action: Action| {    
    let mut r = r.clone();
    let w = w.clone();
    async move {  
      for event in action.events.iter() {   
        if event.paths().any(|(p, _)| p.ends_with(file)) { 
          let ignore = Ignore::new(file).await.unwrap();
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
  we.main().await.into_diagnostic()?;
  Ok(())
}

