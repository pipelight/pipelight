use miette::{Diagnostic, Report};
use pipelight_exec::ExecError;
use pipelight_files::CastError;
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

    #[error(transparent)]
    #[diagnostic(transparent)]
    ExecError(#[from] ExecError),
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
