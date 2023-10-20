// Structs
use utils::git::{Flag, Special};
use crate::services::{Service, FgBg};
use crate::services;
use crate::types::{Commands, DetachableCommands, PostCommands, Trigger};
// use crate::trigger;
// Globals
use std::sync::Arc;
// Watchexec
use watchexec_events::Event;
use watchexec_signals::Signal;
use watchexec_filterer_ignore::IgnoreFilterer;
use watchexec::{
    // Trait
    action::{Action, Outcome},
    config::{InitConfig, RuntimeConfig},
    handler::{PrintDebug},
    Watchexec,
};
use ignore_files::{ IgnoreFilter, IgnoreFile };
// Env
use std::env;
use std::path::{Path};
// Globals
use crate::globals::CLI;
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
pub async fn build() -> Result<(Arc<Watchexec>, RuntimeConfig)> {
  // Default config
  let mut init = InitConfig::default();
  init.on_error(PrintDebug(std::io::stderr()));

  let mut runtime = RuntimeConfig::default();
  let ignore_path = ".pipelight_ignore";
  let filterer = filter_configuration(ignore_path).await?;
  runtime.filterer(Arc::new(filterer));
  // Watch cwd only
  runtime.pathset(vec![env::current_dir().unwrap()]);

  // Create WE instance
  let watchexec = Watchexec::new(init, runtime.clone()).unwrap();
  let w_clone = watchexec.clone();
  let r_clone = runtime.clone();

  runtime.on_action(move |action: Action| {
    let w_clone = w_clone.clone();
    let r_clone = r_clone.clone();

    async move {
      // Self reconfigure on ignore file change
      reconfigure(&w_clone, &r_clone, &action, ignore_path).await.unwrap();
      // Pipeline execution
      watch_trigger().unwrap();
      // Handle Stop signals
			let sigs = action
				.events
				.iter()
				.flat_map(Event::signals)
				.collect::<Vec<_>>();
      if sigs.iter().any(|sig| sig == &Signal::Interrupt) {
        action.outcome(Outcome::Exit);
      } else{
        action.outcome(Outcome::if_running(
            Outcome::DoNothing,
            Outcome::both(Outcome::Clear, Outcome::Start),
        ));
      }
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
pub async fn reconfigure(watchexec: &Arc<Watchexec>, runtime: &RuntimeConfig, action: &Action, ignorefile: &str) -> Result<()> {
  for event in action.events.iter() {
      if event.paths().any(|(p, _)| p.ends_with(ignorefile)) {
        // Set Filter
        let filterer = filter_configuration(ignorefile).await?;
        let mut r = runtime.clone();
        r.filterer(Arc::new(filterer));
        watchexec.reconfigure(r).unwrap();
        break;
      }
  }
  Ok(())
}
/**
Create action filter
 */
pub async fn filter_configuration(path: &str)-> Result<IgnoreFilterer> {
  let path = Path::new(path);
  // Set Filter
  let applies_in = env::current_dir().into_diagnostic()?;
  let file = IgnoreFile {
      path: path.into(),
      applies_in: Some(applies_in.clone()),
      applies_to: None,
  };
  let globs = [".pipelight/*", ".git", ".cargo"];
  let mut filter: IgnoreFilter = IgnoreFilter::empty(applies_in.clone());
  filter.add_globs(&globs, Some(&applies_in)).into_diagnostic()?;
  filter.add_file(&file).await.into_diagnostic()?;
  let filterer = IgnoreFilterer(filter);
  Ok(filterer)
}

/**
Set the watch flag to the triggering env and try to trigger pipelines.
*/
pub fn watch_trigger() -> Result<()> {
  let flag = Some(String::from(&Flag::Special(Special::Watch)));
  let mut args = CLI.lock().unwrap().clone();
  args.commands = Commands::PostCommands(PostCommands::DetachableCommands(
      DetachableCommands::Trigger(Trigger { flag }),
  ));
  Service::new(services::Action::Trigger, Some(args))?.should_detach()?;
  Ok(())
}
