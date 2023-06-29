// Error Handling
use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("js file syntax issue!")]
#[diagnostic(code(json::error))]
pub struct JsonError {
    #[source_code]
    pub src: NamedSource,
    #[label("This bit here")]
    pub bad_bit: SourceSpan,
}

#[derive(Error, Debug, Diagnostic)]
#[error("yaml file syntax issue!")]
#[diagnostic(code(yaml::error))]
pub struct YamlError {
    #[source_code]
    pub src: NamedSource,
    #[label("This bit here")]
    pub bad_bit: SourceSpan,
}

#[derive(Error, Debug, Diagnostic)]
#[error("toml file syntax issue!")]
#[diagnostic(code(yaml::error))]
pub struct TomlError {
    #[source_code]
    pub src: NamedSource,
    #[label("This bit here")]
    pub bad_bit: SourceSpan,
}
