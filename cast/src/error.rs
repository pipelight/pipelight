// Error Handling
use miette::miette;
use miette::{
    Diagnostic,
    IntoDiagnostic,
    MietteHandlerOpts,
    NamedSource,
    Report,
    Result,
    //
    RgbColors,
    SourceOffset,
    SourceSpan,
};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum CastError {
    #[error(transparent)]
    #[diagnostic(transparent)]
    JsonError(#[from] JsonError),
}

#[derive(Error, Diagnostic, Debug)]
#[diagnostic(code(cast::json))]
#[error("Couldn't convert json to Pipelight inner types")]
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

#[derive(Error, Diagnostic, Debug)]
#[diagnostic(code(cast::yaml))]
#[error("Couldn't convert json to Pipelight inner types")]
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
#[derive(Error, Debug, Diagnostic)]
#[error("toml file syntax issue!")]
#[diagnostic(code(yaml::error))]
pub struct TomlError {
    #[source_code]
    pub src: NamedSource,
    #[label("This bit here")]
    pub bad_bit: SourceSpan,
}
pub fn make_handler() -> Result<()> {
    miette::set_hook(Box::new(|_| {
        Box::new(
            MietteHandlerOpts::new()
                .rgb_colors(RgbColors::Never)
                .color(true)
                .unicode(true)
                .terminal_links(true)
                .context_lines(3)
                .with_cause_chain()
                .build(),
        )
    }))?;
    Ok(())
}
