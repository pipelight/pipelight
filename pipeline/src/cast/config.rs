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
    pub fn get() -> Config {
        let em = "pipelight.config.mjs";
        let ts = "pipelight.config.ts";
        let em = Config::load_from_file(&em);
        let ts = Config::load_from_file(&ts);
        let res;
        if ts.is_ok() {
            res = ts;
        } else {
            res = em;
        }
        match res {
            Ok(res) => {
                return res;
            }
            Err(e) => {
                let message = format!("No config file provided");
                println!("{}", message);
                exit(1);
            }
        }
    }
    /// Return the config from .mjs file inside the working dir.
    fn load_from_file(file: &str) -> Result<Config> {
        Config::exists(file)?;
        Config::lint(file)?;
        Config::check(file)?;

        let pwd = current_dir().unwrap();
        let string = format!("{}/{}", &pwd.display().to_string(), file);
        let path = Path::new(&string);

        let executable = "deno eval";
        let script = main_script(file);

        let command = format!("{} {}", executable, script);
        let data = Exec::new().simple(&command)?;
        // println!("{:?}", data);
        let res = serde_json::from_str::<Config>(&data.stdout.clone().unwrap());
        match res {
            Ok(res) => {
                return Ok(res);
            }
            Err(e) => {
                println!("{:?}", e);
                let json_err = JsonError {
                    src: NamedSource::new(file, data.stdout.clone().unwrap()),
                    bad_bit: (e.line(), e.column()).into(),
                };
                let me = Error::from(json_err);
                println!("{:?}", me);
                exit(1);
            }
        }
    }
    /// Ensure config file exists
    fn exists(file: &str) -> Result<bool> {
        let pwd = current_dir().unwrap();
        let string = format!("{}/{}", &pwd.display().to_string(), file);
        let path = Path::new(&string);
        let exist = Path::new(path).exists();
        if !exist {
            let message = "Config file not found.";
            error!("{}", message);
            exit(1);
        } else {
            return Ok(exist);
        }
    }

    /// Ensure that the node.js has no error
    fn lint(file: &str) -> Result<()> {
        // debug!("Linting config file");
        let command = format!("deno lint --quiet {}", file);
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
