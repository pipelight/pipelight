// Error Handling
use miette::Result;

use cli::types::Commands;

// sys
use exec::Process;

// Logger
use log::trace;

// Global
use crate::globals::CLI;

/// Run the command in a detached subprocess
pub fn detach(subcommand: Option<Commands>) -> Result<()> {
    // Run a detached subprocess
    trace!("Create detached subprocess");
    let bin = "pipelight";
    let mut args;
    args = CLI.lock().unwrap().clone();

    args.attach = true;
    if let Some(subcommand) = subcommand {
        args.commands = subcommand
    }

    #[cfg(debug_assertions)]
    let command = format!("cargo run --bin {} {}", &bin, &args);

    #[cfg(not(debug_assertions))]
    let command = format!("{} {}", &bin, &args);

    Process::new(&command).run_detached()?;
    Ok(())
}
