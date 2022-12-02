use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};
use log4rs::init_file;
use std::error::Error;

pub fn set_logger_config() -> Result<(), Box<dyn Error>> {
    let _handle = log4rs::init_file("log4rs.yml", Default::default())?;
    Ok(())
}
