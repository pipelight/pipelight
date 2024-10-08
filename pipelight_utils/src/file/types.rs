// Enum iteration workaround
use strum::EnumIter;

/**
An enum representing the file extention accepted/recognized by pipelight
*/
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, EnumIter, Eq, Ord)]
pub enum FileType {
    #[default]
    // Programming Languages
    TypeScript,
    JavaScript,

    Json,
    // Best file formats
    Toml,
    Tml,

    // Kdl,

    // New gen proprietary file formats
    // Apple configuration as code
    // Pkl,

    // Hashicorp configuration language
    Hcl,

    // Worst file formats
    Yaml,
    Yml,
}
