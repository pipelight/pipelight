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
        let extension: &str = &extension;
        match extension {
            "yaml" => return FileType::Yaml,
            "yml" => return FileType::Yml,
            "toml" => return FileType::Toml,
            "tml" => return FileType::Tml,
            "ts" => return FileType::TypeScript,
            "js" => return FileType::JavaScript,
            _ => {
                let message = format!("Couldn't parse file with extension .{}", extension);
                error!("{}", message);
                exit(1);
            }
        };
    }
}
impl From<&FileType> for String {
    fn from(file_type: &FileType) -> String {
        match file_type {
            FileType::Yaml => return "yaml".to_owned(),
            FileType::Yml => return "yml".to_owned(),
            FileType::Toml => return "toml".to_owned(),
            FileType::Tml => return "tml".to_owned(),
            FileType::TypeScript => return "ts".to_owned(),
            FileType::JavaScript => return "js".to_owned(),
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
        return teleport;
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
        return self.to_owned();
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
            exists = path.clone().exists();
            if exists {
                self.config.file_path = Some(path.display().to_string());
                self.config.directory_path = Some(path.parent().unwrap().display().to_string());
                break;
            }
        }

        // If config file exist break
        // Else recursively call this function in a parent dir
        if exists {
            return self.to_owned();
        } else {
            // if reached git repo root
            if Git::new().exists() {
                if cwd
                    == Git::new()
                        .repo
                        .unwrap()
                        .workdir()
                        .unwrap()
                        .display()
                        .to_string()
                {
                    let message = format!("Couldn't find a config file");
                    error!("{}", message);
                    // println!("{}", message);
                    exit(1);
                    // return Err(Error::msg(message));
                }
            }
            let parent = cwd_path.parent();
            // println!("parent: {}", &parent.unwrap().display());
            if parent.is_some() {
                self.current = parent.unwrap().display().to_string();
                self.search();
            } else {
                // No more accessible parents
                let message = format!("Couldn't find a config file");
                error!("{}", message);
                exit(1);
                // return Err(Error::msg(message));
            }
        }
        return self.to_owned();
    }
}
