// Struct
use crate::services::types::Service;
// Process manipulation
use exec::SelfProcess;
// Globals
use crate::globals::CLI;
// Error Handling
use log::trace;
use miette::Result;

use super::Exec;

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
        if let Some(args) = self.args.clone() {
            if args == origin{

        self.exec()?;
            }else{

            SelfProcess::run_fg_with_cmd(&String::from(&args))?;
            }
        }
        Ok(())
    }
    fn detach(&self) -> Result<()> {
        if let Some(args) = self.args.clone() {
            SelfProcess::run_bg_with_cmd(&String::from(&args))?;
        }
        Ok(())
    }
    fn should_detach(&mut self) -> Result<()> {
        if let Some(args) = self.args.clone() {
            match args.attach {
                Some(true) => {
                    trace!("pipelight process is attached");
                    self.attach()?;
                }
                Some(false) => {
                    trace!("pipelight process is detached");
                    // Exit the detach loop
                    if let Some(e) = self.args.as_mut() {
                        e.attach = Some(true);
                    }
                    self.detach()?;
                }
                None => {
                    trace!("pipelight process is detached");
                    // Exit the detach loop
                    if let Some(e) = self.args.as_mut() {
                        e.attach = Some(true);
                    }
                    self.detach()?;
                }
            };
        }
        Ok(())
    }
}
