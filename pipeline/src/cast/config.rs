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
use utils::teleport::{FileType, Teleport};

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
#[derive(Error, Debug, Diagnostic)]
#[error("yaml file syntax issue!")]
#[diagnostic(code(yaml::error))]
struct YamlError {
    #[source_code]
    src: NamedSource,
    #[label("This bit here")]
    bad_bit: SourceSpan,
}

impl Config {
    pub fn get() -> Result<Config> {
        let file_names: Vec<String> = vec!["pipelight.ts".to_owned(), "pipelight.yml".to_owned()];
        let pwd: String = current_dir().unwrap().display().to_string();

        let file_path = Teleport::new().config_path.unwrap();
        // println!("{}", file_path);
        let res = Config::load_from_file(&file_path);
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

    fn load_from_file(file_path: &str) -> Result<Config> {
        // println!("extensiFileType::from(
        let extension = &Path::new(file_path)
            .extension()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();
        let file_type = FileType::from(extension);
        // println!("{:?}", file_type);
        let config = match file_type {
            FileType::TypeScript | FileType::JavaScript => Config::load_from_file_ts(file_path),
            FileType::Toml | FileType::Tml => Config::load_from_file_tml(file_path),
            FileType::Yaml | FileType::Yml => Config::load_from_file_yml(file_path),
        };
        Ok(config?)
    }

    /// Return the config from given path
    pub fn load_from_file_ts(file_path: &str) -> Result<Config> {
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
                // println!("{:?}", me);
                return Err(me);
                // exit(1);
            }
        }
    }
    fn load_from_file_yml(file_path: &str) -> Result<Config> {
        let executable = "cat";
        let command = format!("{} {}", executable, file_path);
        let data = Exec::new().simple(&command)?;
        // println!("{:?}", data);
        let yml = data.stdout.clone().unwrap();
        let res = serde_yaml::from_str::<Config>(&yml);
        match res {
            Ok(res) => {
                return Ok(res);
            }
            Err(e) => {
                println!("{:?}", e);
                // println!("{}", json);
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
    fn load_from_file_tml(file_path: &str) -> Result<Config> {
        let executable = "cat";
        let command = format!("{} {}", executable, file_path);
        let data = Exec::new().simple(&command)?;
        // println!("{:?}", data);
        let tml = data.stdout.clone().unwrap();
        let res = toml::from_str::<Config>(&tml);
        match res {
            Ok(res) => {
                return Ok(res);
            }
            Err(e) => {
                println!("{:?}", e);
                // println!("{}", json);
                let span: SourceSpan = e.span().unwrap().into();
                let toml_err = YamlError {
                    src: NamedSource::new("config_yaml_output", tml),
                    bad_bit: span,
                };
                let me = Error::from(toml_err);
                println!("{:?}", me);
                exit(1);
            }
        }
    }

    /// Ensure that the node.js has no error
    fn lint(file: &str) -> Result<()> {
        // debug!("Linting config file");
        //
        // reload deno package
        // Exec::new().simple("deno cache --reload npm:pipelight")?;

        let command = format!(
            "deno lint \
            --rules-exclude=no-explicit-any,no-unused-vars \
            --quiet {}",
            file
        );
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
        let command = format!(
            "deno run \
            --allow-net \
            --allow-read \
            --allow-env \
            --allow-run \
            --check --quiet {}",
            file
        );
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
