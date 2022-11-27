#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#[allow(dead_code)]
mod cli;
// mod logger;
mod shell;
mod types;

// use log::{debug, error, info, trace, warn};

fn main() {
    // logger::set_logger_config();
    shell::load_config();
    cli::get_args();
}
