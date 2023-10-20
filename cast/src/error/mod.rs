// Tests
mod test;
// Error Handling
use miette::{Diagnostic, SourceOffset, SourceSpan};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum CastError {
    #[error(transparent)]
    #[diagnostic(transparent)]
    JsonError(#[from] JsonError),
    #[error(transparent)]
    #[diagnostic(transparent)]
    YamlError(#[from] YamlError),
    #[error(transparent)]
    #[diagnostic(transparent)]
    TomlError(#[from] TomlError),
}

/**
A json report type with hint, colors and code span for better pipeline debugging
*/
#[derive(Error, Diagnostic, Debug)]
#[diagnostic(code(cast::json))]
#[error("Serde: Could not convert Json into Rust types")]
pub struct JsonError {
    #[source]
    pub origin: serde_json::Error,
    #[label("here")]
    pub at: SourceSpan,
    #[source_code]
    pub src: String,
}
impl JsonError {
    pub fn new(e: serde_json::Error, src: &str) -> Self {
        JsonError {
            at: SourceSpan::new(
                SourceOffset::from_location(
                    //source
                    src,
                    e.line(),
                    e.column(),
                ),
                1.into(),
            ),
            src: src.to_owned(),
            origin: e,
        }
    }
}

/**
A yaml report type with hint, colors and code span for better pipeline debugging
*/
#[derive(Error, Diagnostic, Debug)]
#[diagnostic(code(cast::yaml))]
#[error("Serde: Could not convert Yaml into Rust types")]
pub struct YamlError {
    #[source]
    pub origin: serde_yaml::Error,
    #[label("here")]
    pub at: SourceSpan,
    #[source_code]
    pub src: String,
}
impl YamlError {
    pub fn new(e: serde_yaml::Error, src: &str) -> Self {
        if let Some(location) = e.location() {
            let line = location.line();
            let column = location.column();
            YamlError {
                at: SourceSpan::new(
                    SourceOffset::from_location(
                        //source
                        src, line, column,
                    ),
                    1.into(),
                ),
                src: src.to_owned(),
                origin: e,
            }
        } else {
            YamlError {
                at: SourceSpan::new(0.into(), 0.into()),
                src: src.to_owned(),
                origin: e,
            }
        }
    }
}

/**
A toml report type with hint, colors and code span for better pipeline debugging
*/
#[derive(Error, Diagnostic, Debug)]
#[diagnostic(code(cast::toml))]
#[error("Serde: Could not convert Toml into Rust types")]
pub struct TomlError {
    #[source]
    pub origin: toml::de::Error,
    #[label("here")]
    pub at: SourceSpan,
    #[source_code]
    pub src: String,
}
impl TomlError {
    pub fn new(e: toml::de::Error, src: &str) -> Self {
        if let Some(span) = e.span() {
            let line = span.start;
            let column = span.end;
            TomlError {
                at: SourceSpan::new(
                    SourceOffset::from_location(
                        //source
                        src, line, column,
                    ),
                    1.into(),
                ),
                src: src.to_owned(),
                origin: e,
            }
        } else {
            TomlError {
                at: SourceSpan::new(0.into(), 0.into()),
                src: src.to_owned(),
                origin: e,
            }
        }
    }
}
