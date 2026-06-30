use crate::Style;
use convert_case::{Case, Casing};
use pipelight_utils::FileType;

impl From<&String> for Style {
    fn from(e: &String) -> Self {
        let e: &String = &e.to_case(Case::Snake);
        match serde_plain::from_str(e) {
            Ok(res) => res,
            Err(_) => Style::default(),
        }
    }
}
impl From<&Style> for String {
    fn from(e: &Style) -> Self {
        match serde_plain::to_string::<Style>(e) {
            Ok(res) => res,
            Err(_) => "toml".to_owned(),
        }
    }
}

impl From<&Style> for FileType {
    fn from(e: &Style) -> Self {
        match *e {
            Style::Toml => FileType::Toml,
            Style::Hcl => FileType::Hcl,
            Style::Yaml => FileType::Yaml,
            Style::Json => FileType::Json,

            Style::TsHelpers => FileType::TypeScript,
            Style::Js => FileType::JavaScript,
            Style::Ts => FileType::TypeScript,
        }
    }
}
impl From<&FileType> for Style {
    fn from(e: &FileType) -> Self {
        match *e {
            FileType::Toml => Style::Toml,
            FileType::Tml => Style::Toml,
            FileType::Hcl => Style::Hcl,
            FileType::Yaml => Style::Yaml,
            FileType::Yml => Style::Yaml,
            FileType::Json => Style::Json,

            FileType::TypeScript => Style::Ts,
            FileType::JavaScript => Style::Js,
        }
    }
}
