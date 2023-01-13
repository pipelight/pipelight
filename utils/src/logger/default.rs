use super::Logger;
pub use log::LevelFilter;

impl Default for Logger {
    fn default() -> Self {
        let directory = ".pipelight/logs";
        let mut logger = Logger {
            directory: directory.to_owned(),
            level: LevelFilter::Trace,
            handle: None,
        };
        return logger;
    }
}
impl Logger {
    pub fn new() -> Self {
        Self::default()
    }
}
