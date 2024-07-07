use miette::{Diagnostic, Report};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum PipelightError {
    #[error(transparent)]
    #[diagnostic(code(pipelight::io::error))]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    #[diagnostic(code(pipelight::io::error))]
    LibError(#[from] LibError),
}

/**
A config error with help.
Can be recursively chained.
*/
#[derive(Debug, Error, Diagnostic)]
#[error("{}", message)]
#[diagnostic(code(pipelight::lib::error))]
pub struct LibError {
    pub message: String,
    #[diagnostic_source]
    pub origin: Report,
    #[help]
    pub help: String,
}
impl LibError {
    pub fn new(message: &str, help: &str, e: Report) -> Self {
        LibError {
            help: help.to_owned(),
            message: message.to_owned(),
            origin: e,
        }
    }
}
