// Enum iteration workaround
use strum::EnumIter;

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
