use crate::Style;
use convert_case::{Case, Casing};
use pipelight_utils::files::FileType;

impl From<&String> for Style {
    fn from(e: &String) -> Self {
        let e: &String = &e.to_case(Case::Kebab);
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
            Err(_) => "objects".to_owned(),
        }
    }
}

impl From<&Style> for FileType {
    fn from(e: &Style) -> Self {
        match *e {
            Style::Objects => FileType::TypeScript,
            Style::Helpers => FileType::TypeScript,
            Style::Javascript => FileType::JavaScript,
            Style::Toml => FileType::Toml,
            Style::Yaml => FileType::Yaml,
        }
    }
}
impl From<&FileType> for Style {
    fn from(e: &FileType) -> Self {
        match *e {
            FileType::TypeScript => Style::Objects,
            FileType::JavaScript => Style::Objects,
            FileType::Toml => Style::Toml,
            FileType::Tml => Style::Toml,
            FileType::Yaml => Style::Yaml,
            FileType::Yml => Style::Yaml,
            _ => Style::Toml,
        }
    }
}
