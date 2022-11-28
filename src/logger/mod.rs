use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};
use log4rs::init_file;

pub fn set_logger_config() -> Result<(), SetLoggerError> {
    let _handle = log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    Ok(())
}
