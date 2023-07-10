// Standard libs
use std::env;

use super::git::Git;
use std::path::Path;
// Enum workaround
use std::string::ToString;
use strum::{EnumIter, IntoEnumIterator};
// Error Handling
use log::{debug, error, trace, warn};
use miette::{miette, Diagnostic, Error, IntoDiagnostic, NamedSource, Report, Result, SourceSpan};
use std::process::exit;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, PartialOrd, EnumIter, Eq, Ord)]
pub enum FileType {
    TypeScript,
    JavaScript,
    Toml,
    Tml,
    Yaml,
    Yml,
}

impl From<&String> for FileType {
    fn from(extension: &String) -> FileType {
        let extension: &str = extension;
        match extension {
            "yaml" => FileType::Yaml,
            "yml" => FileType::Yml,
            "toml" => FileType::Toml,
            "tml" => FileType::Tml,
            "ts" => FileType::TypeScript,
            "js" => FileType::JavaScript,
            _ => {
                let message = format!("Couldn't parse file with extension .{}", extension);
                error!("{}", message);
                exit(1);
            }
        }
    }
}
impl From<&FileType> for String {
    fn from(file_type: &FileType) -> String {
        match file_type {
            FileType::Yaml => "yaml".to_owned(),
            FileType::Yml => "yml".to_owned(),
            FileType::Toml => "toml".to_owned(),
            FileType::Tml => "tml".to_owned(),
            FileType::TypeScript => "ts".to_owned(),
            FileType::JavaScript => "js".to_owned(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Config {
    pub preffix: String,
    pub directory_path: Option<String>,
    pub file_path: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            preffix: "pipelight".to_owned(),
            directory_path: None,
            file_path: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Teleport {
    pub config: Config,
    // Path from which process triggerd (cwd)
    pub origin: String,
    // Cwd
    pub current: String,
}

impl Default for Teleport {
    fn default() -> Self {
        let mut teleport = Teleport {
            origin: env::current_dir().unwrap().display().to_string(),
            current: env::current_dir().unwrap().display().to_string(),
            config: Config::default(),
        };
        teleport.search();
        teleport
    }
}

impl Teleport {
    pub fn new() -> Self {
        Teleport::default()
    }
    pub fn teleport(&mut self) -> Self {
        let cwd = env::current_dir().unwrap().display().to_string();
        if cwd == self.origin {
            env::set_current_dir(self.config.clone().directory_path.unwrap()).unwrap();
        }
        if cwd == self.config.clone().directory_path.unwrap() {
            env::set_current_dir(self.origin.clone()).unwrap();
        }
        self.to_owned()
    }
    /// Recursively search a file throught parent dir and return path if exists
    pub fn search(&mut self) -> Self {
        let mut cwd = self.current.clone();
        cwd.push('/');

        let cwd_path = Path::new(&cwd);
        let message = "Couldn't find a configuration file";

        // Loop through file types
        let mut exists = false;
        for file_type in FileType::iter() {
            let extension = String::from(&file_type);
            let file_str = format!("{}{}.{}", cwd, self.config.preffix, extension).to_owned();
            let path = Path::new(&file_str);
            exists = path.exists();
            if exists {
                self.config.file_path = Some(path.display().to_string());
                self.config.directory_path = Some(path.parent().unwrap().display().to_string());
                break;
            }
        }

        // If config file exist break
        // Else recursively call this function in a parent dir
        if !exists {
            // if reached git repo root
            if Git::new().exists()
                && cwd
                    == Git::new()
                        .repo
                        .unwrap()
                        .workdir()
                        .unwrap()
                        .display()
                        .to_string()
            {
                let message = "Couldn't find a config file".to_owned();
                error!("{}", message);
                // println!("{}", message);
                exit(1);
                // return Err(Error::msg(message));
            }
            let parent = cwd_path.parent();
            // println!("parent: {}", &parent.unwrap().display());
            if let Some(parent) = parent {
                self.current = parent.display().to_string();
                self.search();
            } else {
                // No more accessible parents
                let message = "Couldn't find a config file".to_owned();
                error!("{}", message);
                exit(1);
                // return Err(Error::msg(message));
            }
        }
        self.to_owned()
    }
}
