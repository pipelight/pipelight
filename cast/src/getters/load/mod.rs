mod markup;
mod typescript;

// Error Handling
use log::warn;
use miette::Result;

use std::path::Path;
use utils::teleport::FileType;

use crate::Config;
impl Config {
    /// Set the appropriated method to load the config according to the FileType
    /// (the file extension .ts, .toml, .yml...)
    pub fn file(file_path: &str, args: Option<Vec<String>>) -> Result<Config> {
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
        let mut config = match file_type {
            FileType::TypeScript | FileType::JavaScript => Config::ts(file_path, args)?,
            FileType::Toml | FileType::Tml => Config::tml(file_path)?,
            FileType::Yaml | FileType::Yml => Config::yml(file_path)?,
        };
        config.strict_check()
    }
    /**
     * Enforce pipelines rules
     * - No whitespaces in pipeline names
     *
     */
    fn strict_check(&mut self) -> Result<Config> {
        if let Some(pipelines) = self.pipelines.clone() {
            for pipeline in pipelines {
                if pipeline.name.contains(char::is_whitespace) {
                    warn!("The pipeline {} has an invalide name", pipeline.name);
                    println!("{:#?}", pipeline);
                }
            }
        }
        Ok(self.to_owned())
    }
}
