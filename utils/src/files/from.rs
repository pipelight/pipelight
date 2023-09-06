use super::FileType;
use log::error;

// Error Handling
use std::process::exit;

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
