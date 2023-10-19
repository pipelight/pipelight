// Structs
use utils::git::{Flag, Special};
use crate::services::{Service, FgBg};
use crate::services;
use crate::types::{Cli, Commands, DetachableCommands, PostCommands};
use crate::types::{Trigger};
// use crate::trigger;
// Globals
use std::sync::Arc;
// Watchexec
use watchexec_signals::Signal;
use watchexec::{
    fs,
    action,
    action::{Action, Outcome},
    event::{Event, Priority, Tag},
    config::{InitConfig, RuntimeConfig},
    handler::{Handler as _, PrintDebug},
    Watchexec,
};
use utils::files::Ignore;
// Env
use std::process::exit;
use std::env;
use std::process;
use std::future::Future;
// Globals
use crate::globals::CLI;
// Error handling
use log::{error, info, trace};
use miette::{IntoDiagnostic, Result, Diagnostic};
use thiserror::Error; 

use async_priority_channel as priority;

use tokio::{
	sync::{mpsc, watch},
	time::sleep,
};

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
pub async fn build() -> Result<()> {
  // Default config
  let mut init = InitConfig::default();
  init.on_error(PrintDebug(std::io::stderr()));

  /*  Working DataSet declaration */
  // Action
  let mut action_data = action::WorkingData::default();
  // Set Filter
  let ignorefile = ".pipelight_ignore";
  let ignore = Ignore::new(ignorefile).unwrap();
  action_data.filterer = Arc::new(ignore);

  // Filesystem
  let mut fs_data = fs::WorkingData::default();
	fs_data.pathset = vec![env::current_dir().unwrap().into()];
	let (wd_s, wd_r) = watch::channel(fs_data);
	wd_s.send(fs_data.clone()).into_diagnostic()?;


	let (ev_s, ev_r) = priority::bounded::<Event, Priority>(1024);
	let (er_s, mut er_r) = mpsc::channel(64);

  /* Logging */
  // info log
	tokio::spawn(async move {
		while let Ok((event, priority)) = ev_r.recv().await {
      info!("event ({priority:?}): {event:?}");
		}
	});
  // error log
	tokio::spawn(async move {
		while let Some(error) = er_r.recv().await {
			error!("error: {error}");
		}
	});

  // interrupt
	tokio::spawn(async move {
		while let Ok((event, priority)) = ev_r.recv().await {
      info!("event {priority:?}: {event:?}");
			if event.tags.contains(&Tag::Signal(Signal::Terminate)) {
				exit(0);
			}
		}
	});

  // reconfigure
	tokio::spawn(async move {
		while let Ok((event, priority)) = ev_r.recv().await {
      if event.paths().any(|(p, _)| p.ends_with(ignorefile)) {
        // Set Filter
        ignore = Ignore::new(ignorefile).unwrap();
        action_data.filterer = Arc::new(ignore);
      }
    }
	});
  
  tokio::spawn(async move {
    let sigs = action
      .events
      .iter()
      .flat_map(Event::signals)
      .collect::<Vec<_>>();
    if sigs.iter().any(|sig| sig == &Signal::Interrupt) {
      action.outcome(Outcome::Exit);
    } else {
      // Self reconfigure on ignore file change
      reconfigure(&w_clone, &r_clone, &action, ignorefile).unwrap();
      // Pipeline execution
      watch_trigger().unwrap();
      // Handle Stop signals
      action.outcome(Outcome::if_running(
          Outcome::DoNothing,
          Outcome::both(Outcome::Clear, Outcome::Start),
      ));
    }
  });
  Ok(())
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
  let flag = Some(String::from(&Flag::Special(Special::Watch)));
  let mut args = CLI.lock().unwrap().clone();
  args.commands = Commands::PostCommands(PostCommands::DetachableCommands(
      DetachableCommands::Trigger(Trigger { flag }),
  ));
  Service::new(services::Action::Trigger, Some(args))?.should_detach()?;
  Ok(())
}
