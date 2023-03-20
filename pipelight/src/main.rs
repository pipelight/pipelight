#![allow(unused_variables)]
// #![allow(unused_imports)]
#![allow(unused_must_use)]
// use performance::*;
use log::error;
use shared::cli;
#[allow(dead_code)]
use std::error::Error;
use std::process::exit;

// Error Handling
// use miette::{miette, Diagnostic, Error, IntoDiagnostic, NamedSource, Report, Result, SourceSpan};
// use thiserror::Error;

fn main() {
    handler().unwrap_or_else(|e| {
        error!("{}", e);
        exit(1)
    })
}
fn handler() -> Result<(), Box<dyn Error>> {
    cli::get_args()?;
    Ok(())
}
