use super::FileType;
use log::error;

/**
Convert the file extension into a FileType struct
*/
impl From<&String> for FileType {
    fn from(extension: &String) -> FileType {
        let extension: &str = extension;
        match extension {
            "ts" => FileType::TypeScript,
            "js" => FileType::JavaScript,
            "toml" => FileType::Toml,
            "tml" => FileType::Tml,

            "hcl" => FileType::Hcl,
            "pkl" => FileType::Pkl,

            "yaml" => FileType::Yaml,
            "yml" => FileType::Yml,
            _ => {
                let message = format!("Couldn't parse file with extension .{}", extension);
                let _hint = "Assuming default typescript file";
                error!("{}", message);
                FileType::TypeScript
            }
        }
    }
}
/**
Convert the FileType struct into a file  extension string
*/
impl From<&FileType> for String {
    fn from(file_type: &FileType) -> String {
        match file_type {
            FileType::TypeScript => "ts".to_owned(),
            FileType::JavaScript => "js".to_owned(),
            FileType::Toml => "toml".to_owned(),
            FileType::Tml => "tml".to_owned(),

            FileType::Hcl => "hcl".to_owned(),
            FileType::Pkl => "pkl".to_owned(),

            FileType::Yaml => "yaml".to_owned(),
            FileType::Yml => "yml".to_owned(),
        }
    }
}
