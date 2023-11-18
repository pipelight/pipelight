use miette::{Diagnostic, Report, Result};
use thiserror::Error;
/**
A config error with help.
Can be recursively chained.
*/
#[derive(Debug, Error, Diagnostic)]
#[error("{}", message)]
#[diagnostic(code(pipelight::api::error))]
pub struct LibError {
    pub message: String,
    #[diagnostic_source]
    pub origin: Report,
    #[help]
    pub help: String,
}
impl LibError {
    pub fn new(message: &str, help: &str, e: Report) -> Self {
        let err = LibError {
            help: help.to_owned(),
            message: message.to_owned(),
            origin: e,
        };
        // println!("{:#?}", err);
        err
    }
}
