#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#[allow(dead_code)]
use lib::actions;
use lib::cli;
use lib::logger;
use lib::shell;
use lib::types;
use log::{debug, error, info, trace, warn};

fn main() {
    logger::set_logger_config();
    shell::load_config();
    cli::get_args();
}
