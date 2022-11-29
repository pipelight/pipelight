#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
mod actions;
mod cli;
mod logger;
mod shell;
#[allow(dead_code)]
mod types;

use log::{debug, error, info, trace, warn};

fn main() {
    logger::set_logger_config();
    shell::load_config();
    cli::get_args();
}
