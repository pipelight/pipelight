#![allow(unused_variables)]
// #![allow(unused_imports)]
#![allow(unused_must_use)]
#[allow(dead_code)]
// use performance::*;

// Logger
use log::error;

// Command Line
use shared::cli;
use std::env;

use std::process::exit;

// Error Handling
use miette::Result;

fn main() {
    handler().unwrap_or_else(|e| {
        error!("{}", e);
        exit(1)
    })
}
fn handler() -> Result<()> {
    let mut raw_args = env::args().collect::<Vec<String>>();
    raw_args.remove(0);
    cli::get_args(raw_args)?;

    Ok(())
}
