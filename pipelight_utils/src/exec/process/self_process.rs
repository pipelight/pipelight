// Struct
use crate::{
    error::PipelightError,
    exec::{Process, SelfProcess},
};
// Error Handling
use miette::Result;

impl SelfProcess {
    /**
    Clone the pipelight instance and detach the clone.

    The instance spawned by the command line can exit whithout killing it's child.
    Thus the clone can run and persist in the background even on tty close.
    */
    pub fn run_bg_with_cmd(cmd_args: &str) -> Result<(), PipelightError> {
        // global vars
        let bin = "pipelight";

        // Dev env or production env
        #[cfg(debug_assertions)]
        let command = format!("cargo run --bin {} {}", &bin, &cmd_args);
        #[cfg(not(debug_assertions))]
        let command = format!("{} {}", &bin, &cmd_args);

        // Run a detached subprocess
        Process::new(&command).run_detached()?;
        Ok(())
    }
    /**
    Clone the pipelight instance and detach the clone.

    The instance spawned by the command line can exit whithout killing it's child.
    Thus the clone can run and persist in the background even on tty close.
    */
    pub fn run_fg_with_cmd(cmd_args: &str) -> Result<(), PipelightError> {
        // global vars
        let bin = "pipelight";

        // Dev env or production env
        #[cfg(debug_assertions)]
        let command = format!("cargo run --bin {} {}", &bin, &cmd_args);
        #[cfg(not(debug_assertions))]
        let command = format!("{} {}", &bin, &cmd_args);

        // Run a detached subprocess
        Process::new(&command).run_inherit()?;
        Ok(())
    }
}
