// Enum workaround
use std::string::ToString;
use strum::{EnumIter, IntoEnumIterator};
// Standard libs
use std::env;

#[derive(Debug, Clone, PartialEq, PartialOrd, EnumIter, Eq, Ord)]
pub enum FileType {
    TypeScript,
    JavaScript,
    Toml,
    Tml,
    Yaml,
    Yml,
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd)]
/// Internal values to browse the fs
pub struct Internal {
    /// Config file dir path and file path if founded.
    pub directory_path: Option<String>,
    pub file_path: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NaiveFileInfo {
    pub preffix: String,
    pub path: Option<String>,
}
impl Default for NaiveFileInfo {
    fn default() -> Self {
        NaiveFileInfo {
            preffix: "pipelight".to_owned(),
            path: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Teleport {
    pub internal: Internal,
    pub file_info: NaiveFileInfo,
    /// Process origin path and current path
    pub origin: String,
    pub current: String,
}
impl Default for Teleport {
    fn default() -> Self {
        Teleport {
            internal: Internal::default(),
            origin: env::current_dir().unwrap().display().to_string(),
            current: env::current_dir().unwrap().display().to_string(),
            file_info: NaiveFileInfo::default(),
        }
    }
}
impl Teleport {
    pub fn new() -> Self {
        Teleport::default()
    }
}
