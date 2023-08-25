use super::detach::detach;
use exec::Process;
use log::{debug, trace};
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

/// Launch attached thread
pub fn watch_in_thread() -> Result<()> {
    let thread = thread::spawn(move || {
        //Action
        watch().unwrap()
    });
    thread.join().unwrap();
    Ok(())
}
/// Filter pipeline by trigger and run
pub fn watch() -> Result<()> {
    let config = Config::get()?;

    // Set triggering env
    let env = Trigger::flag(Some(Flag::Special(Special::Watch)))?;

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
        flag: Some(String::from(&env.action.unwrap())),
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
        if parsed_cmd.is_ok()
            && parsed_cmd.into_diagnostic()?.commands
                == types::Commands::Watch(types::Watch { commands: None })
            && process.cwd() == env::current_dir().into_diagnostic()?
            && pid != &get_current_pid().unwrap()
        {
            let message = "a watcher is already running on this project";
            //     let hint = "no need to re run another watcher";
            return Err(Error::msg(message));
        }
    }
    Ok(())
}
/// Filter pipeline by trigger and run
pub fn create_watcher() -> Result<()> {
    let subcommand = types::Commands::Watch(types::Watch { commands: None });
    if can_watch().is_ok() {
        detach(Some(subcommand))?;
    }
    Ok(())
}
/// Remove the running watcher instance
pub fn destroy_watcher() -> Result<()> {
    let mut sys = System::new_all();
    sys.refresh_all();
    for (pid, process) in sys.processes() {
        let parsed_cmd = types::Cli::try_parse_from(process.cmd());
        if parsed_cmd.is_ok()
            && parsed_cmd.into_diagnostic()?.commands
                == types::Commands::Watch(types::Watch { commands: None })
            && process.cwd() == env::current_dir().into_diagnostic()?
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
    Ok(())
}
