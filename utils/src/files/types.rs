// Enum iteration workaround
use strum::EnumIter;
// Ignore file
use ignore_files::IgnoreFilter;

/**
An enum representing the file extention accepted/recognized by pipelight
*/
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, EnumIter, Eq, Ord)]
pub enum FileType {
    #[default]
    TypeScript,
    JavaScript,
    Toml,
    Tml,
    Yaml,
    Yml,
}

/**
Ignore file filtering utility
*/
#[derive(Debug, Clone)]
pub struct Ignore {
    pub filter: IgnoreFilter,
}
