use super::detach::detach;
use exec::Process;
use log::{debug, error, info, trace, warn};
use pipeline::{Config, Trigger};
use std::thread;
use utils::git::{Flag, Special};
use utils::teleport::Teleport;

use crate::interface::types;

// sys
use clap::Parser;
use rustix::process::{getpgid, kill_process_group, test_kill_process_group, Pid, Signal};
use std::env;
use sysinfo::{get_current_pid, PidExt, ProcessExt, System, SystemExt};

// Global
use crate::case::CLI;

// Error Handling
use miette::{Error, IntoDiagnostic, Result};

pub fn launch(attach: bool) -> Result<()> {
    trace!("Create detached subprocess");
    match attach {
        true => {
            // Lauch in attached thread
            trace!("Run pipeline in attached thread");
            watch_in_thread()?;
        }
        false => detach(None)?,
    }
    Ok(())
}

/// Filter pipeline by trigger and run
pub fn create_watcher() -> Result<()> {
    let config = Config::get()?;
    let mut env = Trigger::env()?;

    env.action = Some(Flag::Special(Special::Watch));

    // Guard
    if config.pipelines.is_none() {
        let message = "No pipeline found";
        debug!("{}", message);
        return Ok(());
    }

    // Set global triggering flag/action to "watch"
    let env = Trigger::flag(Flag::Special(Special::Watch))?;
    info!("{:#?}", env);
    let mut args;
    unsafe {
        args = (*CLI).clone();
    }

    let subcommand = types::Commands::Watch(types::Watch {
        commands: Some(types::WatchCommands::Kill),
    });

    unsafe {
        (*CLI) = args;
    }

    if can_watch().is_ok() {
        detach(Some(subcommand))?;
    }

    Ok(())
}

/// Filter pipeline by trigger and run
pub fn watch() -> Result<()> {
    let config = Config::get()?;

    // Set global triggering flag/action to "watch"
    Trigger::flag(Flag::Special(Special::Watch))?;

    // Guard
    if config.pipelines.is_none() {
        let message = "No pipeline found";
        debug!("{}", message);
        return Ok(());
    }

    let bin = "pipelight";
    let mut args;
    unsafe {
        args = (*CLI).clone();
    }
    args.attach = true;
    args.commands = types::Commands::Trigger(types::Trigger {
        flag: Some("watch".to_owned()),
    });

    #[cfg(debug_assertions)]
    let action = format!("cargo run --bin {} {}", &bin, &args);

    #[cfg(not(debug_assertions))]
    let action = format!("{} {}", &bin, &args);

    let command = format!("watchexec -w {} {}", Teleport::new().origin, &action);

    if can_watch().is_ok() {
        Process::new(&command).simple()?;
    }
    Ok(())
}

// Test if an instance of (pipelight watch /watchexec is already
// watching the current working directory
pub fn can_watch() -> Result<()> {
    let mut sys = System::new_all();
    sys.refresh_all();
    for (pid, process) in sys.processes() {
        let parsed_cmd = types::Cli::try_parse_from(process.cmd());
        if parsed_cmd.is_ok() {
            if parsed_cmd.into_diagnostic()?.commands
                == types::Commands::Watch(types::Watch {
                    commands: Some(types::WatchCommands::Kill),
                })
            {
                if process.cwd() == env::current_dir().into_diagnostic()?
                    && pid != &get_current_pid().unwrap()
                {
                    let message = "a watcher is already running on this project";
                    //     let hint = "no need to re run another watcher";
                    return Err(Error::msg(message));
                }
            }
        }
    }
    Ok(())
}
/// Remove the running watcher instance
pub fn destroy_watcher() -> Result<()> {
    let mut sys = System::new_all();
    sys.refresh_all();
    for (pid, process) in sys.processes() {
        let parsed_cmd = types::Cli::try_parse_from(process.cmd());
        if parsed_cmd.is_ok() {
            if parsed_cmd.into_diagnostic()?.commands
                == types::Commands::Watch(types::Watch {
                    commands: Some(types::WatchCommands::Kill),
                })
            {
                if process.cwd() == env::current_dir().into_diagnostic()?
                    && pid != &get_current_pid().unwrap()
                {
                    // Kill watcher and subprocesses
                    let pid = process.pid().as_u32();
                    unsafe {
                        let pgid = getpgid(Pid::from_raw(pid)).into_diagnostic()?;
                        if test_kill_process_group(pgid).is_ok() {
                            kill_process_group(pgid, Signal::Term).into_diagnostic()?;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

/// Launch attached thread
pub fn watch_in_thread() -> Result<()> {
    let thread = thread::spawn(move || {
        //Action
        watch().unwrap()
    });
    thread.join().unwrap();
    Ok(())
}
