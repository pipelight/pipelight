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
    Yaml,
    Yml,
    Toml,
    Tml,
    TypeScript,
    JavaScript,
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
                let message = format!("Couldn't parse config file with extension .{}", extension);
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
pub struct Teleport {
    pub root: Option<String>,
    pub config_path: Option<String>,
    pub cwd: Option<String>,
}

impl Default for Teleport {
    fn default() -> Self {
        let mut cwd = env::current_dir().unwrap().display().to_string();
        let cwd = Some(cwd);
        let root: Option<String>;
        let config_path: Option<String>;
        let res = Teleport::search("pipelight", &cwd.clone().unwrap());
        if res.is_ok() {
            let path = res.unwrap().clone();
            println!("{}", path);
            root = Some(Path::new(&path).parent().unwrap().display().to_string());
            config_path = Some(path);
        } else {
            root = None;
            config_path = None;
        }
        return Teleport {
            root: root,
            config_path: config_path,
            cwd: cwd,
        };
    }
}

impl Teleport {
    pub fn new() -> Self {
        Teleport::default()
    }
    pub fn teleport(&mut self) {
        let cwd = env::current_dir().unwrap().display().to_string();
        let root = self.clone().root.unwrap();
        let current = self.clone().cwd.unwrap();
        if cwd != root || cwd != current {
            return;
        }
        if cwd == root {
            env::set_current_dir(&current).unwrap();
        }
        if cwd == current {
            env::set_current_dir(&root).unwrap();
        }
    }
    /// Recursively search a file throught parent dir
    pub fn search(app_name: &str, cwd: &str) -> Result<String> {
        let mut cwd = cwd.to_owned();
        cwd.push('/');

        let message = "Couldn't find a configuration file";

        // Convert args to path
        let mut exists = false;
        let mut file_str: String = format!("{}{}", cwd, app_name).to_owned();
        let binding = file_str.clone();
        let mut file_path = Path::new(&binding);
        let mut dir = Path::new(&cwd);

        // println!("{}", &file_path.display());

        for file_type in FileType::iter() {
            let extension = String::from(&file_type);
            let path_str = format!("{}{}.{}", cwd, app_name, extension).to_owned();
            file_str = path_str.clone();
            let path = Path::new(&path_str);
            exists = path.clone().exists();
            if exists {
                println!("str: {}", &path_str);
                break;
            }
        }
        // Config try get
        if exists {
            println!("f: {}", &file_str);
            return Ok(file_str);
            // Load config from str -> Path
        } else {
            // Reached git repo root
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
            let parent = dir.parent();
            println!("parent: {}", &parent.unwrap().display());
            if parent.is_some() {
                let new_path = Teleport::search(app_name, &parent.unwrap().display().to_string())?;
                return Ok(new_path);
            } else {
                // No more accessible parents
                let message = format!("Couldn't find a config file");
                error!("{}", message);
                exit(1);
                // return Err(Error::msg(message));
            }
        }
    }
}
