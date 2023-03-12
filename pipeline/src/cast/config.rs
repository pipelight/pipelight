use super::Config;
use exec::Exec;
use log::{debug, error, trace, warn};

// standard lib
use std::env::current_dir;
// use std::error::Error;
use std::fmt;
use std::path::Path;
use std::process::exit;
use typescript::{main_script, TYPES};
use utils::teleport::Teleport;

// Error Handling
use miette::{miette, Diagnostic, Error, IntoDiagnostic, NamedSource, Report, Result, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("js file syntax issue!")]
#[diagnostic(code(json::error))]
struct JsonError {
    #[source_code]
    src: NamedSource,
    #[label("This bit here")]
    bad_bit: SourceSpan,
}

impl Config {
    pub fn get() -> Result<Config> {
        let file_name: String = "pipelight.config.ts".to_owned();
        let pwd: String = current_dir().unwrap().display().to_string();

        let path_str = Teleport::search(&file_name, &pwd)?;
        let res = Config::load_from_file(&path_str);
        match res {
            Ok(res) => {
                return Ok(res);
            }
            Err(e) => {
                let message = format!("Error in config file:\n{}", e);
                return Err(Error::msg(message));
                // println!("{}", message);
                // exit(1);
            }
        }
    }

    /// Return the config from given path
    fn load_from_file(file_path: &str) -> Result<Config> {
        // Fail safe guards
        Config::lint(file_path)?;
        Config::check(file_path)?;

        let executable = "deno eval";
        let script = main_script(file_path);

        let command = format!("{} {}", executable, script);
        let data = Exec::new().simple(&command)?;
        // println!("{:?}", data);
        let json = data.stdout.clone().unwrap();
        let res = serde_json::from_str::<Config>(&json);
        match res {
            Ok(res) => {
                return Ok(res);
            }
            Err(e) => {
                println!("{:?}", e);
                // println!("{}", json);
                let span: SourceSpan = (e.line(), e.column()).into();
                let json_err = JsonError {
                    src: NamedSource::new("config_json_output", json),
                    bad_bit: span,
                };
                let me = Error::from(json_err);
                println!("{:?}", me);
                exit(1);
            }
        }
    }

    /// Ensure that the node.js has no error
    fn lint(file: &str) -> Result<()> {
        // debug!("Linting config file");
        let command = format!("deno lint --rules-exclude=no-explicit-any --quiet {}", file);
        let data = Exec::new().simple(&command)?;
        if data.stdout.is_none() {
            if data.stderr.is_none() {
                Ok(())
            } else {
                let message = format!("{}", data.stderr.unwrap());
                Err(Error::msg(message))
            }
        } else {
            if data.stderr.is_none() {
                Ok(())
            } else {
                let message = format!("{}", data.stderr.unwrap());
                Err(Error::msg(message))
            }
        }
    }
    /// Ensure Typescript typing
    fn check(file: &str) -> Result<()> {
        // debug!("Linting config file");
        let command = format!("deno run --allow-read --check --quiet {}", file);
        let data = Exec::new().simple(&command)?;
        if data.stdout.is_none() {
            if data.stderr.is_none() {
                Ok(())
            } else {
                let message = format!("{}", data.stderr.unwrap());
                Err(Error::msg(message))
            }
        } else {
            if data.stderr.is_none() {
                Ok(())
            } else {
                let message = format!("{}", data.stderr.unwrap());
                Err(Error::msg(message))
            }
        }
    }
}
