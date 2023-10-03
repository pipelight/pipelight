// Struct
use cli::types::Commands;
// Error Handling
use miette::Result;
// sys
use exec::Process;
// Logger
use log::trace;
// Global
use cli::globals::CLI;

/**
Clone the pipelight instance and detach the clone.

The instance spawned by the command line can exit whithout killing it's child.
Thus the clone can run and persist in the background even on tty close.
*/

pub fn detach() -> Result<()> {
    // global vars
    let bin = "pipelight";
    let mut args;
    args = CLI.lock().unwrap().clone();

    // Dev env or production env
    #[cfg(debug_assertions)]
    let command = format!("cargo run --bin {} {}", &bin, &args);
    #[cfg(not(debug_assertions))]
    let command = format!("{} {}", &bin, &args);

    // Run a detached subprocess
    Process::new(&command).run_detached()?;
    Ok(())
}

/**
Inspect the parsed command line arguments (CLI global, attach flag)
and determine wheteher to detach the subprocess or not.
*/
pub fn should_detach() -> Result<()> {
    let mut args;
    args = CLI.lock().unwrap().clone();
    match args.attach {
        true => {
            trace!("pipelight process is attached");
        }
        false => {
            trace!("detach pipelight process");
            // Exit the detach loop
            // for the to be detached process
            args.attach = true;
            *CLI.lock().unwrap() = args;
            // Detach process
            detach()?;
        }
    }
    Ok(())
}
