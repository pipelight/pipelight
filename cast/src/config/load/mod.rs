// Tests
mod test;

mod markup;
mod rules;
mod typescript;
// Structs
use crate::Config;
// Filesystem
use std::path::Path;
use utils::files::FileType;
// Error Handling
use miette::Result;

impl Config {
    /// Set the appropriated method to load the config according to the FileType
    /// (the file extension .ts, .toml, .yml...)
    pub fn load(file_path: &str, args: Option<Vec<String>>) -> Result<Config> {
        // println!("extensiFileType::from(
        let extension = &Path::new(file_path)
            .extension()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();

        let file_type = FileType::from(extension);
        let mut config = match file_type {
            FileType::TypeScript | FileType::JavaScript => Config::ts(file_path, args)?,
            FileType::Toml | FileType::Tml => Config::tml(file_path)?,
            FileType::Yaml | FileType::Yml => Config::yml(file_path)?,
        };
        config.strict_check()
    }
}
