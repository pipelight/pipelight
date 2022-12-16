#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
// use performance::*;
use shared::cli;
use shared::exec;
#[allow(dead_code)]
use shared::logger;
use shared::types::Config;
use std::error::Error;
use std::process::exit;

fn main() {
    handler().unwrap_or_else(|a| exit(1))
}
fn handler() -> Result<(), Box<dyn Error>> {
    cli::get_args()?;
    Ok(())
}
