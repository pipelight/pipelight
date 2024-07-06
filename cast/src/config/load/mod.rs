// Tests
mod test;

mod markup;
mod rules;
mod typescript;
// Structs
use crate::Config;
// Filesystem
use std::path::Path;
use pipelight_utils::files::FileType;
// Error Handling
use miette::Result;

impl Config {
    /**
    Choose the appropriated method to load the config file
    according to the file extension(.ts, .toml, .yml...).

    Arguments:
      - file_path is the config file path
      - args are only to be used with scripting language (typescript) to pass args to the underlying script.

    Languages coming next after v1.0.0:
      - Rust, Hcl, Kcl, Python...
    */
    pub fn load(file_path: &str, args: Option<Vec<String>>) -> Result<Config> {
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
