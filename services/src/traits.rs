// Struct
use crate::types::Service;
use cli::types::Cli;
// Process manipulation
use exec::SelfProcess;
// Error Handling
use miette::Result;
// Global vars

pub trait Detach {
    /**
    Fork action/process end send to background
    */
    fn detach(&mut self) -> Result<()>;
    fn should_detach(&mut self) -> Result<()>;
}

pub trait Parser {
    /**
    Take the command line arguments and recycle every arguments that can be
    to launch the service with same args (verbosity, flags...) as the main process.
    */
    fn cmd(&mut self, args: Cli) -> Result<()>;
}

impl Detach for Service {
    fn detach(&mut self) -> Result<()> {
        // SelfProcess::run_bg_with_cmd(args)?;
        Ok(())
    }
    /**
    Inspect the parsed command line arguments (CLI global, attach flag)
    and determine wheteher to detach the subprocess or not.

    Arguments:
    - args: Optional args for the to be detached command.

    */
    fn should_detach(&mut self) -> Result<()> {
        // let args = CLI.lock().unwrap();
        //
        // let command = string_to_command(&String::from("pipelight watch"))?;
        // args.commands = command.clone();
        //
        // println!("{:#?}", command);
        // println!("{}", args);
        //
        // match args.attach.clone() {
        // true => {
        // trace!("pipelight process is attached");
        // Ok(())
        // }
        // false => {
        // trace!("detach pipelight process");
        // Exit the detach loop
        // for the to be detached process
        // (*CLI.lock().unwrap()).attach = true;
        // Detach process
        // detach(args)?;
        // Ok(())
        // }
        // }
        Ok(())
    }
}
