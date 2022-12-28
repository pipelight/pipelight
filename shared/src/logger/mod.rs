use crate::types::logs::{PipelineLog, PipelineStatus};
pub use log::Level::{Debug, Trace};
pub use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};
use log4rs;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Handle;
use project_root::get_project_root;
use rev_buf_reader::RevBufReader;
use std::error::Error;
use std::fs;
use std::fs::{read_to_string, File};
use std::io::{self, BufRead, Read, Write};
use std::path::Path;
use uuid::{uuid, Uuid};
pub mod config;

/// Create logs directory
pub fn ensure_log_directory() -> Result<(), Box<dyn Error>> {
    let path = Path::new("./.pipelight/logs");
    fs::create_dir_all(path)?;
    Ok(())
}

/// Set loggers and return handler to change logLevels at runtime
pub fn set_logger(level: LevelFilter) -> Result<Handle, Box<dyn Error>> {
    ensure_log_directory()?;
    let config = config::set(level)?;
    // use handle to change logger configuration at runtime
    let handle = log4rs::init_config(config).unwrap();
    Ok(handle)
}

/// Delete logs directory
pub fn clear_logs() -> Result<(), Box<dyn Error>> {
    let path = Path::new(".pipelight/logs");
    fs::remove_dir_all(path)?;
    Ok(())
}

/// Pretty print logs from json log file
pub fn pretty_logs() -> Result<(), Box<dyn Error>> {
    let paths = fs::read_dir(".pipelight/logs").unwrap();
    for res in paths {
        let dir_entry = res?;
        let json = last_lines(&dir_entry.path())?;
        let pipeline = serde_json::from_str::<PipelineLog>(&json)?;
        println!("{}", pipeline);
    }
    Ok(())
}
pub fn json_logs() {
    // let file_path = ".pipelight/logs/pipelines.json.log";
    // let contents = read_to_string(file_path).expect("Should have been able to read the file");
    // println!("{}", contents);
}
pub fn raw_logs() {
    // let file_path = ".pipelight/logs/pipelines.raw.log";
    // let contents = read_to_string(file_path).expect("Should have been able to read the file");
    // println!("{}", contents);
}

fn last_lines(path: &Path) -> Result<String, Box<dyn Error>> {
    let file = File::open(path)?;
    let buf = RevBufReader::new(file);
    let mut lines = buf.lines();
    let last_line = lines.next().unwrap().unwrap();
    Ok(last_line)
}
