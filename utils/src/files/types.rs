// Enum workaround
use strum::EnumIter;

#[derive(Debug, Clone, PartialEq, PartialOrd, EnumIter, Eq, Ord)]
pub enum FileType {
    TypeScript,
    JavaScript,
    Toml,
    Tml,
    Yaml,
    Yml,
}
