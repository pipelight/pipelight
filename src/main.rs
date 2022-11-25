#![allow(unused_variables)]
#![allow(unused_imports)]

mod logger;
use log::{debug, error, info, trace, warn};

fn main() {
    logger::set_logger_config();
    info!("this is a debug");
}
