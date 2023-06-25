use super::Config;

// Logger
use log::{debug, error, trace, warn};

// Exec
use exec::Process;

// Standard lib
use std::env::current_dir;
use std::path::Path;
use std::process::exit;

use super::typescript::main_script;
use std::fmt;
use utils::teleport::{FileType, Teleport};

// Error Handling
use crate::error::{JsonError, TomlError, YamlError};
use miette::{miette, Diagnostic, Error, IntoDiagnostic, NamedSource, Report, Result, SourceSpan};

impl Config {
    pub fn get(file: Option<String>, args: Option<Vec<String>>) -> Result<Config> {
        let pwd: String = current_dir().unwrap().display().to_string();

        let default_file_path: String = Teleport::new().config.file_path.unwrap();

        let res: Result<Config>;
        if file.is_some() {
            // Canonicalize file path
            if Path::new(&file.clone().unwrap()).exists() {
                let file_path: String = Path::new(&file.unwrap())
                    .canonicalize()
                    .into_diagnostic()?
                    .display()
                    .to_string();
                res = Config::load_from_file(&file_path, args);
            } else {
                let message = format!("Couldn't find a config file: {}", &file.unwrap());
                return Err(Error::msg(message));
            }
        } else {
            res = Config::load_from_file(&default_file_path, args);
        }

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
    fn load_from_file(file_path: &str, args: Option<Vec<String>>) -> Result<Config> {
        // println!("extensiFileType::from(
        let extension = &Path::new(file_path)
            .extension()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();

        // println!("{}", extension);

        let file_type = FileType::from(extension);
        // println!("{:?}", file_type);
        let config = match file_type {
            FileType::TypeScript | FileType::JavaScript => {
                Config::load_from_file_ts(file_path, args)
            }
            FileType::Toml | FileType::Tml => Config::load_from_file_tml(file_path),
            FileType::Yaml | FileType::Yml => Config::load_from_file_yml(file_path),
        };
        Ok(config?)
    }

    /// Return a Config struct from a provided typescript file path
    pub fn load_from_file_ts(file_path: &str, args: Option<Vec<String>>) -> Result<Config> {
        // Fail safe guards
        Config::lint(file_path)?;
        Config::check(file_path, args.clone())?;

        let executable = "deno eval";
        let script = main_script(file_path);
        let command;
        if args.is_some() {
            command = format!("{} {} -- {}", executable, script, args.unwrap().join(" "));
        } else {
            command = format!("{} {}", executable, script);
        }
        let p = Process::new(&command).simple()?;
        let json = p.state.stdout.clone().unwrap();
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
                // println!("{:?}", me);
                return Err(me);
                // exit(1);
            }
        }
    }
    /// Return a Config struct from a provided toml file path
    fn load_from_file_tml(file_path: &str) -> Result<Config> {
        let executable = "cat";
        let command = format!("{} {}", executable, file_path);
        let p = Process::new(&command).simple()?;

        let tml = p.state.stdout.clone().unwrap();
        let res = toml::from_str::<Config>(&tml);
        match res {
            Ok(res) => {
                return Ok(res);
            }
            Err(e) => {
                println!("{:?}", e);
                // println!("{}", json);
                let span: SourceSpan = e.span().unwrap().into();
                let toml_err = TomlError {
                    src: NamedSource::new("config_toml_output", tml),
                    bad_bit: span,
                };
                let me = Error::from(toml_err);
                println!("{:?}", me);
                exit(1);
            }
        }
    }
    /// Return a Config struct from a provided yaml file path
    fn load_from_file_yml(file_path: &str) -> Result<Config> {
        let executable = "cat";
        let command = format!("{} {}", executable, file_path);
        let p = Process::new(&command).simple()?;

        let yml = p.state.stdout.clone().unwrap();
        let res = serde_yaml::from_str::<Config>(&yml);
        match res {
            Ok(res) => {
                return Ok(res);
            }
            Err(e) => {
                println!("{:?}", e);
                let span: SourceSpan =
                    (e.location().unwrap().line(), e.location().unwrap().column()).into();
                let yaml_err = YamlError {
                    src: NamedSource::new("config_yaml_output", yml),
                    bad_bit: span,
                };
                let me = Error::from(yaml_err);
                println!("{:?}", me);
                exit(1);
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
        let p = Process::new(&command).simple()?;
        if p.state.stdout.is_none() {
            if p.state.stderr.is_none() {
                Ok(())
            } else {
                let message = format!("{}", p.state.stderr.unwrap());
                Err(Error::msg(message))
            }
        } else {
            if p.state.stderr.is_none() {
                Ok(())
            } else {
                let message = format!("{}", p.state.stderr.unwrap());
                Err(Error::msg(message))
            }
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

        let p = Process::new(&command).simple()?;

        if p.state.stdout.is_none() {
            if p.state.stderr.is_none() {
                Ok(())
            } else {
                let message = format!("{}", p.state.stderr.unwrap());
                Err(Error::msg(message))
            }
        } else {
            if p.state.stderr.is_none() {
                Ok(())
            } else {
                let message = format!("{}", p.state.stderr.unwrap());
                Err(Error::msg(message))
            }
        }
    }
}
