#![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(dead_code)]
#![allow(unused_must_use)]

// use performance::*;

use cli::get_args;

use log::error;
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
    get_args()?;
    Ok(())
}
