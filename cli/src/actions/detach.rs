// Error Handling
use miette::Result;

// sys
use exec::Process;

// Logger
use log::trace;

// Global
use crate::case::CLI;

/// Run the command in a detached subprocess
pub fn detach() -> Result<()> {
    // Run a detached subprocess
    trace!("Create detached subprocess");
    let bin = "pipelight";
    let mut args;
    unsafe {
        args = (*CLI).clone();
    }
    args.attach = true;

    #[cfg(debug_assertions)]
    let command = format!("cargo run --bin {} {}", &bin, &args);

    #[cfg(not(debug_assertions))]
    let command = format!("{} {}", &bin, &args);

    Process::new(&command).detached()?;
    Ok(())
}
