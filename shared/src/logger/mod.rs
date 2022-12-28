use crate::types::logs::PipelineLog;
pub use log::Level;
pub use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};
use log4rs;
use log4rs::Handle;
use rev_buf_reader::RevBufReader;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
pub mod config;

/// Create logs directory
pub struct Read {}
impl Read {
    fn last_lines(path: &Path) -> Result<String, Box<dyn Error>> {
        let file = File::open(path)?;
        let buf = RevBufReader::new(file);
        let mut lines = buf.lines();
        let last_line = lines.next().unwrap().unwrap();
        Ok(last_line)
    }
}
pub struct Logs {
    path: String,
    level: LevelFilter,
}
impl Logs {
    pub fn new() -> Self {
        return Logs {
            path: "./.pipelight/logs".to_owned(),
            level: LevelFilter::Trace,
        };
    }
    /// Ensure log directory
    pub fn ensure(&self) -> Result<(), Box<dyn Error>> {
        let path = Path::new(&self.path);
        fs::create_dir_all(path)?;
        Ok(())
    }
    /// Set loggers and return handler to change logLevels at runtime
    pub fn set(&self) -> Result<Handle, Box<dyn Error>> {
        self.ensure()?;
        let config = config::set(self.level)?;
        // use handle to change logger configuration at runtime
        let handle = log4rs::init_config(config).unwrap();
        Ok(handle)
    }
    /// Delete logs directory
    pub fn clear() -> Result<(), Box<dyn Error>> {
        let path = Path::new(".pipelight/logs");
        fs::remove_dir_all(path)?;
        Ok(())
    }
    /// Pretty print logs from json log file
    pub fn pretty() -> Result<(), Box<dyn Error>> {
        let paths = fs::read_dir(".pipelight/logs").unwrap();
        for res in paths {
            let dir_entry = res?;
            let json = Read::last_lines(&dir_entry.path())?;
            let pipeline = serde_json::from_str::<PipelineLog>(&json)?;
            println!("{}", pipeline);
        }
        Ok(())
    }
    pub fn json() -> Result<(), Box<dyn Error>> {
        let paths = fs::read_dir(".pipelight/logs").unwrap();
        for res in paths {
            let dir_entry = res?;
            let json = Read::last_lines(&dir_entry.path())?;
            let pipeline = serde_json::from_str::<PipelineLog>(&json)?;
            println!("{:?}", pipeline);
        }
        Ok(())
    }
}
