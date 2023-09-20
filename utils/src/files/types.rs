// Enum workaround
use strum::EnumIter;

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
