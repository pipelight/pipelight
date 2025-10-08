// Error Handling
use miette::{Error, Result};

// Exec
use pipelight_error::{CastError, JsonError, LibError, PipelightError};
use pipelight_exec::Process;

mod script;
use script::import_script;

use crate::Config;

impl Config {
    /// Return a Config struct from a provided typescript file path
    pub fn ts(file_path: &str, args: Option<Vec<String>>) -> Result<Config> {
        // Fail safe guards
        Config::lint(file_path)?;
        Config::check(file_path, args.clone())?;

        let executable = "deno eval";
        let script = import_script(file_path);
        let command = if args.is_some() {
            format!("{} {} -- {}", executable, script, args.unwrap().join(" "))
        } else {
            format!("{} {}", executable, script)
        };
        let p = Process::new().stdin(&command).term().run()?;

        let mut string = "".to_owned();
        if let Some(stdout) = p.io.stdout {
            string = stdout;
        } else {
            let err = LibError {
                message: p.io.stderr.unwrap(),
                help: "".to_owned(),
            };
            return Err(err.into());
        }
        let res = serde_json::from_str::<Config>(&string);
        match res {
            Ok(res) => Ok(res),
            Err(e) => {
                let err = CastError::JsonError(JsonError::new(e, &string));
                Err(err.into())
            }
        }
    }
    /// Check if the deno script contains syntax errors
    fn lint(file: &str) -> Result<()> {
        // debug!("Linting config file");
        let command = format!(
            "deno lint \
            --rules-exclude=no-explicit-any,no-unused-vars,no-import-prefix \
            --quiet {}",
            file
        );
        let mut p = Process::new().stdin(&command).term().to_owned();
        p.run()?;
        if p.io.stderr.is_none() {
            Ok(())
        } else {
            let message = p.io.stderr.unwrap();
            let help = "Fix typos in order to run the piplines".to_owned();
            let err = PipelightError::LibError(LibError { message, help });
            Err(err.into())
        }
    }
    /// Run the script to detect runtime errors
    fn check(file: &str, args: Option<Vec<String>>) -> Result<()> {
        // debug!("Linting config file");
        let mut command = format!(
            "deno run \
            --allow-net \
            --allow-read \
            --allow-env \
            --allow-run \
            --check \
            --quiet \
            {}",
            file,
        );
        if args.is_some() {
            command = format!("{} {}", command, args.unwrap().join(" "));
        }

        let mut p = Process::new().stdin(&command).term().to_owned();
        p.run()?;

        if p.io.stderr.is_none() {
            Ok(())
        } else {
            let message = p.io.stderr.unwrap();
            let help = "Fix errors in order to run the piplines".to_owned();
            let err = PipelightError::LibError(LibError { message, help });
            Err(err.into())
        }
    }
}
