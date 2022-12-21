#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
// use performance::*;
use log::{debug, error, info, trace, warn};
use shared::cli;
use shared::exec;
#[allow(dead_code)]
use shared::logger;
use shared::types::config::Config;
use std::error::Error;
use std::process::exit;

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
