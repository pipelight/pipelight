use miette::set_hook;
use miette::{
    Context, Diagnostic, GraphicalReportHandler, LabeledSpan, MietteHandlerOpts,
    NarratableReportHandler, ReportHandler, Result, RgbColors, Severity, SourceCode, SourceSpan,
};
use std::fmt;
use thiserror::Error;

#[macro_export]
macro_rules! code_location {
    () => {
        PipeError()
        format!("file:'{}' line:'{}'", file!(), line!())
    };
}

#[derive(Error, Diagnostic, Debug)]
pub enum CliError {
    #[error(transparent)]
    #[diagnostic(code(pipelight::io_error))]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    #[diagnostic(code(pipeline::type_error))]
    PipeError(#[from] PipeError),
}

#[derive(Error, Diagnostic, Debug, Clone)]
#[error("pipeline::definition")]
pub struct PipeError {
    #[label("here")]
    pub at: SourceSpan,
    #[source_code]
    pub src: String,
    pub file: String,
}
impl Default for PipeError {
    fn default() -> Self {
        PipeError {
            file: file!().to_owned(),
            at: SourceSpan::new(1.into(), 3.into()),
            src: "mysrting\n is cool\n and all”".to_owned(),
        }
    }
}
impl PipeError {
    pub fn new(file: &str) -> Self {
        PipeError {
            file: file.to_owned(),
            at: SourceSpan::new(1.into(), 3.into()),
            src: "mysrting\n is cool\n and all”".to_owned(),
        }
    }
}

pub fn make_handler() -> Result<()> {
    miette::set_hook(Box::new(|_| {
        Box::new(
            MietteHandlerOpts::new()
                .rgb_colors(RgbColors::Never)
                .color(true)
                .unicode(true)
                .terminal_links(true)
                .build(),
        )
    }))?;
    Ok(())
}
