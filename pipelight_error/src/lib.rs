use miette::{Diagnostic, Report, SourceOffset, SourceSpan};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum PipelightError {
    #[error(transparent)]
    #[diagnostic(code(pipelight::io::error))]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    #[diagnostic(transparent)]
    WrapError(#[from] WrapError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    LibError(#[from] LibError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    CastError(#[from] CastError),
}

/**
A config error with help higher origin
Can be recursively chained.
*/
#[derive(Debug, Error, Diagnostic)]
#[error("{}", message)]
#[diagnostic(code(pipelight::wrap::error))]
pub struct WrapError {
    pub message: String,
    #[diagnostic_source]
    pub origin: Report,
    #[help]
    pub help: String,
}

/**
A root cause error with no inner origin
*/
#[derive(Debug, Error, Diagnostic)]
#[error("{}", message)]
#[diagnostic(code(pipelight::lib::error))]
pub struct LibError {
    pub message: String,
    #[help]
    pub help: String,
}

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

    #[error(transparent)]
    #[diagnostic(transparent)]
    HclError(#[from] HclError),
}

/**
A JSON report type with hint, colors and code span.
For better configuration file debugging
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
                1,
            ),
            src: src.to_owned(),
            origin: e,
        }
    }
}

/**
A TOML report type with hint, colors and code span.
For better configuration file debugging
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
                    1,
                ),
                src: src.to_owned(),
                origin: e,
            }
        } else {
            TomlError {
                at: SourceSpan::new(0.into(), 0),
                src: src.to_owned(),
                origin: e,
            }
        }
    }
}

/**
A Hcl report type with hint, colors and code span.
For better configuration file debugging
*/
#[derive(Error, Diagnostic, Debug)]
#[diagnostic(code(cast::hcl))]
#[error("Serde: Could not convert Hcl into Rust types")]
pub struct HclError {
    #[source]
    pub origin: hcl::Error,
    #[label("here")]
    pub at: SourceSpan,
    #[source_code]
    pub src: String,
}
impl HclError {
    pub fn new(e: hcl::Error, src: &str) -> Self {
        match e {
            hcl::Error::Parse(e) => {
                let line = e.location().line();
                let column = e.location().column();
                HclError {
                    at: SourceSpan::new(
                        SourceOffset::from_location(
                            //source
                            src, line, column,
                        ),
                        1,
                    ),
                    src: src.to_owned(),
                    origin: hcl::Error::from(e),
                }
            }
            _ => HclError {
                at: SourceSpan::new(0.into(), 0),
                src: src.to_owned(),
                origin: e,
            },
        }
    }
}

/**
A YAML report type with hint, colors and code span.
For better configuration file debugging
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
                    1,
                ),
                src: src.to_owned(),
                origin: e,
            }
        } else {
            YamlError {
                at: SourceSpan::new(0.into(), 0),
                src: src.to_owned(),
                origin: e,
            }
        }
    }
}
