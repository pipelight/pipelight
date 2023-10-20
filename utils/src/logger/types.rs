pub use log::LevelFilter;
use log4rs::Handle;

#[derive(Debug, Clone)]
pub struct Logger {
    pub handle: Option<Handle>,
    pub pipelines: LogInfo,
    pub internals: LogInfo,
}
#[derive(Debug, Clone)]
pub struct LogInfo {
    pub file_info: Option<LogFile>,
    pub pattern: String,
    pub name: String,
    pub level: LevelFilter,
}
#[derive(Debug, Clone)]
pub struct LogFile {
    pub directory: String,
    pub name: String,
}
