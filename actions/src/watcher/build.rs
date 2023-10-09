// Structs
use utils::git::{Flag, Special};
use workflow::types::{Trigger};
use crate::trigger;
// Globals
use std::sync::Arc;
// Watchexec
use watchexec_events::Event;
use watchexec::{
    action::{Action, Outcome},
    config::{InitConfig, RuntimeConfig},
    handler::{Handler as _, PrintDebug},
    Watchexec,
};
use utils::files::Ignore;
// Env
use std::env;
// Error handling
use miette::{IntoDiagnostic, Result, Diagnostic};
use thiserror::Error; 


// Watchexec - Bug fix Struct
#[derive(Debug, Error, Diagnostic)]
#[error("stub")] 
struct MietteStub;

/**
Build an appropriate watcher that:
- self reconfigures on ignore file change
- ignores pipelight generated tmp files
- can trigger pipelines
*/
pub fn build() -> Result<(Arc<Watchexec>, RuntimeConfig)> {
  // Default config
  let mut init = InitConfig::default();
  init.on_error(PrintDebug(std::io::stderr()));

  let mut runtime = RuntimeConfig::default();

  // Set Filter
  // Parse ignore file into watchexec filter
  let ignorefile = ".pipelight_ignore";
  let ignore = Ignore::new(ignorefile)?;
  runtime.filterer(Arc::new(ignore));

  // Watch cwd only
  runtime.pathset(vec![env::current_dir().unwrap()]);

  // Create WE instance
  let watchexec = Watchexec::new(init, runtime.clone())?;

  let w_clone = watchexec.clone();
  let r_clone = runtime.clone();

  runtime.on_action(move |action: Action| {
    let w_clone = w_clone.clone();
    let r_clone = r_clone.clone();
    async move {
      // Reconfigure
      reconfigure(&w_clone, &r_clone, &action, ignorefile).unwrap();
      // Pipeline execution
      watch_trigger().unwrap();

      action.outcome(Outcome::if_running(
          Outcome::DoNothing,
          Outcome::both(Outcome::Clear, Outcome::Start),
      ));
      Ok(())
      // (not normally required! ignore this when implementing)
      as std::result::Result<_, MietteStub>
    }
  });

  Ok((watchexec,runtime))
}


/**
Self reconfigure when the IgnoreFile changes.
*/
pub fn reconfigure(watchexec: &Arc<Watchexec>, runtime: &RuntimeConfig, action: &Action, ignorefile: &str) -> Result<()> {
  for event in action.events.iter() {
      if event.paths().any(|(p, _)| p.ends_with(ignorefile)) {
        let ignore = Ignore::new(ignorefile).unwrap();
        // Set Filter
        let mut r = runtime.clone();
        r.filterer(Arc::new(ignore));
        watchexec.reconfigure(r).unwrap();
        break;
      }
  }
  Ok(())
}

/**
Set the watch flag to the triggering env and try to trigger pipelines.
*/
pub fn watch_trigger() -> Result<()> {
  Trigger::flag(Some(Flag::Special(Special::Watch)))?;
  // trigger::launch()?;
  Ok(())
}
