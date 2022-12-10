#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
use log::{debug, error, info, trace, warn};
use shared::actions;
use shared::cli;
use shared::exec;
#[allow(dead_code)]
use shared::logger;
use shared::types::Config;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    logger::set_logger()?;
    cli::get_args();
    Ok(())
}
