#![allow(unused_variables)]
// #![allow(unused_imports)]
#![allow(unused_must_use)]
// use performance::*;
use log::error;
use shared::cli;
#[allow(dead_code)]
use std::process::exit;

// Error Handling
use miette::Result;
// use std::error::Error;

fn main() {
    handler().unwrap_or_else(|e| {
        error!("{}", e);
        exit(1)
    })
}
fn handler() -> Result<()> {
    cli::get_args()?;
    Ok(())
}
