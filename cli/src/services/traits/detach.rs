// Struct
use crate::services::types::Service;
use crate::types::Attach;
// Process manipulation
use pipelight_exec::Process;
use serde::{Deserialize, Serialize};
// Globals
use crate::globals::CLI;
// Error Handling
use log::trace;
use miette::Result;

use super::Exec;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct PipelightBin;

impl PipelightBin {
    /**
    Clone the pipelight instance and detach the clone.

    The instance spawned by the command line can exit whithout killing it's child.
    Thus the clone can run and persist in the background even on tty close.
    */
    pub fn args(cmd_args: &str) -> Process {
        // global vars
        let bin = "pipelight";

        // Dev env or production env
        #[cfg(debug_assertions)]
        let command = format!("cargo run --bin {} {}", &bin, &cmd_args);
        #[cfg(not(debug_assertions))]
        let command = format!("{} {}", &bin, &cmd_args);

        // Run a detached subprocess
        Process::new(&command)
    }
}

pub trait FgBg {
    /**
    Fork action/process end send to background
    */
    fn detach(&self) -> Result<()>;
    /**
    Fork action/process end keep in foreground
    */
    fn attach(&self) -> Result<()>;
    /**
    Inspect the parsed command line arguments (CLI global, attach flag)
    and determine whether to detach the subprocess or not.
    */
    fn should_detach(&mut self) -> Result<()>;
}

impl FgBg for Service {
    fn attach(&self) -> Result<()> {
        let origin = CLI.lock().unwrap().clone();
        // Guard
        if let Some(args) = self.args.clone() {
            if args == origin {
                self.exec()?;
            } else {
                PipelightBin::args(&String::from(&args)).run_piped()?;
            }
        }
        Ok(())
    }
    fn detach(&self) -> Result<()> {
        if let Some(args) = self.args.clone() {
            PipelightBin::args(&String::from(&args)).run_detached_term()?;
        }
        Ok(())
    }
    fn should_detach(&mut self) -> Result<()> {
        if let Some(args) = self.args.clone() {
            // println!("{:#?}", args.attach);
            if let Some(attach) = args.attach {
                match Attach::from(&attach) {
                    Attach::True => {
                        trace!("Subprocess is attached");
                        self.attach()?;
                    }
                    Attach::False => {
                        trace!("Subprocess is detached");
                        // Exit the detach loop
                        if let Some(e) = self.args.as_mut() {
                            e.attach = Some(String::from(&Attach::True));
                        }
                        self.detach()?;
                    }
                };
            } else {
                if let Some(e) = self.args.as_mut() {
                    e.attach = Some(String::from(&Attach::True));
                }
                self.detach()?;
            }
        }
        Ok(())
    }
}
