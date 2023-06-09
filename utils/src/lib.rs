// Rules
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(non_upper_case_globals)]
#[allow(dead_code)]
// External Imports
use project_root::get_project_root;
use rev_buf_reader::RevBufReader;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
// Internal Imports
pub mod git;
pub mod logger;
pub mod teleport;

// Error Handling
use miette::{miette, Diagnostic, Error, IntoDiagnostic, NamedSource, Report, Result, SourceSpan};
use thiserror::Error;
// use std::error::Error;

/// Return project root path as string
pub fn get_root() -> Result<String> {
    let root = get_project_root().into_diagnostic()?;
    let to_str_result = root.to_str();
    match to_str_result {
        Some(res) => return Ok(res.to_owned()),
        None => {
            let message = "Internal error: Couldn't find project root";
            // warn!("{}", message);
            return Err(Error::msg(message));
        }
    };
}

/// Read last line of each log file
pub fn read_last_line(path: &Path) -> Result<String> {
    let file = File::open(path).into_diagnostic()?;
    let buf = RevBufReader::new(file);
    let mut lines = buf.lines();
    let last_line = lines.next().unwrap().unwrap();
    Ok(last_line)
}
