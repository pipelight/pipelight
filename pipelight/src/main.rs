#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
// use performance::*;
use shared::actions;
use shared::cli;
use shared::exec;
#[allow(dead_code)]
use shared::logger;
use shared::types::Config;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    cli::get_args();
    Ok(())
}
