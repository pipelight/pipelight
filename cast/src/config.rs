use super::Config;

// Exec
use exec::Process;

// Standard lib
// use std::env::current_dir;
use std::path::Path;
use std::process::exit;

use super::typescript::main_script;
use utils::teleport;
use utils::teleport::{FileType, Teleport};

// Error Handling
use crate::error::{JsonError, TomlError, YamlError};
use miette::{Error, IntoDiagnostic, NamedSource, Result, SourceSpan};

impl Config {
    /// Browse through the filesystem to find the config file
    /// Immediately set the cwd to the config file location if found
    pub fn get(file: Option<String>, args: Option<Vec<String>>) -> Result<Config> {
        // let pwd: String = current_dir().unwrap().display().to_string();
        let mut teleport = Teleport::new();
        if file.is_some() {
            let file = file.unwrap();
            teleport.file(&file)?;
            teleport.search()?;
        } else {
            // Search default file
            teleport.preffix("pipelight")?;
            teleport.search()?;
        }
        let res = Config::load_from_file(&teleport.internal.file_path.clone().unwrap(), args);
        match res {
            Ok(res) => {
                teleport.teleport();
                Ok(res)
            }
            Err(e) => {
                let message = format!("Error in config file:\n{}", e);
                Err(Error::msg(message))
            }
        }
    }
    /// Browse through the filesystem to find the config file
    /// Immediately set the cwd to the config file location if found
    pub fn get_with_teleport(
        file: Option<String>,
        args: Option<Vec<String>>,
    ) -> Result<(Config, Teleport)> {
        // let pwd: String = current_dir().unwrap().display().to_string();
        let mut teleport = Teleport::new();
        if file.is_some() {
            let file = file.unwrap();
            teleport.file(&file)?;
            teleport.search()?;
        } else {
            // Search default file
            teleport.preffix("pipelight")?;
            teleport.search()?;
        }
        let res = Config::load_from_file(&teleport.internal.file_path.clone().unwrap(), args);
        match res {
            Ok(res) => {
                teleport.teleport();
                Ok((res, teleport))
            }
            Err(e) => {
                let message = format!("Error in config file:\n{}", e);
                Err(Error::msg(message))
            }
        }
    }
    /// Set the appropriated method to load the config according to the FileType
    /// (the file extension .ts, .toml, .yml...)
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
        match file_type {
            FileType::TypeScript | FileType::JavaScript => {
                Config::load_from_file_ts(file_path, args)
            }
            FileType::Toml | FileType::Tml => Config::load_from_file_tml(file_path),
            FileType::Yaml | FileType::Yml => Config::load_from_file_yml(file_path),
        }
    }

    /// Return a Config struct from a provided typescript file path
    pub fn load_from_file_ts(file_path: &str, args: Option<Vec<String>>) -> Result<Config> {
        // Fail safe guards
        Config::lint(file_path)?;
        Config::check(file_path, args.clone())?;

        let executable = "deno eval";
        let script = main_script(file_path);
        let command = if args.is_some() {
            format!("{} {} -- {}", executable, script, args.unwrap().join(" "))
        } else {
            format!("{} {}", executable, script)
        };
        let p = Process::new(&command).simple()?;
        let json = p.state.stdout.unwrap();
        let res = serde_json::from_str::<Config>(&json);
        match res {
            Ok(res) => Ok(res),
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
                Err(me)
                // exit(1);
            }
        }
    }
    /// Return a Config struct from a provided toml file path
    fn load_from_file_tml(file_path: &str) -> Result<Config> {
        let executable = "cat";
        let command = format!("{} {}", executable, file_path);
        let p = Process::new(&command).simple()?;

        let tml = p.state.stdout.unwrap();
        let res = toml::from_str::<Config>(&tml);
        match res {
            Ok(res) => Ok(res),
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

        let yml = p.state.stdout.unwrap();
        let res = serde_yaml::from_str::<Config>(&yml);
        match res {
            Ok(res) => Ok(res),
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
        if p.state.stderr.is_none() {
            Ok(())
        } else {
            let message = p.state.stderr.unwrap();
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

        let p = Process::new(&command).simple()?;

        if p.state.stderr.is_none() {
            Ok(())
        } else {
            let message = p.state.stderr.unwrap();
            Err(Error::msg(message))
        }
    }
}
