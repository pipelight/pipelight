#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#[allow(dead_code)]
mod logger;
mod sh;

use log::{debug, error, info, trace, warn};

fn main() {
    logger::set_logger_config();
    info!("tyrust");
    sh::ts_node()
}
