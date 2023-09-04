// Rules
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(non_upper_case_globals)]
// #![allow(dead_code)]

// External Imports
use rev_buf_reader::RevBufReader;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
// Internal Imports
pub mod git;
pub mod logger;
pub mod teleport;

// Error Handling
use miette::{IntoDiagnostic, Result};

/// Read last line of each log file
pub fn read_last_line(path: &Path) -> Result<String> {
    let file = File::open(path).into_diagnostic()?;
    let buf = RevBufReader::new(file);
    let mut lines = buf.lines();
    let last_line = lines.next().unwrap().unwrap();
    Ok(last_line)
}
