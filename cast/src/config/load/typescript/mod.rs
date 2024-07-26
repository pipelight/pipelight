// Error Handling
use crate::error::JsonError;
use miette::{Error, Result};

// Exec
use pipelight_utils::exec::Process;

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
        let mut p = Process::new(&command);
        p.run_piped()?;
        let json = p.io.stdout.unwrap();
        let res = serde_json::from_str::<Config>(&json);
        match res {
            Ok(res) => Ok(res),
            Err(e) => {
                let err = JsonError::new(e, &json);
                Err(err.into())
            }
        }
    }
    /// Check if the deno script contains syntax errors
    fn lint(file: &str) -> Result<()> {
        // debug!("Linting config file");
        let command = format!(
            "deno lint \
            --rules-exclude=no-explicit-any,no-unused-vars \
            --quiet {}",
            file
        );
        let mut p = Process::new(&command);
        p.run_piped()?;
        if p.io.stderr.is_none() {
            Ok(())
        } else {
            let message = p.io.stderr.unwrap();
            Err(Error::msg(message))
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

        let mut p = Process::new(&command);
        p.run_piped()?;

        if p.io.stderr.is_none() {
            Ok(())
        } else {
            let message = p.io.stderr.unwrap();
            Err(Error::msg(message))
        }
    }
}
