use crate::types::Path;
pub use log::Level::{Debug, Trace};
pub use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};
use log4rs;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Handle;
use project_root::get_project_root;
use std::error::Error;

/// Return logger config with chosen verbosity level
pub fn set_logger_config(level: LevelFilter) -> Result<Config, Box<dyn Error>> {
    let shell_pattern = "{d(%Y-%m-%d %H:%M:%S)} | {h({l}):5.5} | {f}:{L} — \n{m}{n}\n";
    let pattern = "{d(%Y-%m-%d %H:%M:%S)} | {h({l}):5.5} | {f}:{L} — {m}{n}";
    let body = "{m}{n}";
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build();
    let internal_appender = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build(".pipelight/logs/internal.log")
        .unwrap();
    let pipeline_raw_appender = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(shell_pattern)))
        .build(".pipelight/logs/pipelines.raw.log")
        .unwrap();
    let pipeline_json_appender = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(body)))
        .build(".pipelight/logs/pipelines.json.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("internal", Box::new(internal_appender)))
        .appender(Appender::builder().build("pipeline_raw", Box::new(pipeline_raw_appender)))
        .appender(Appender::builder().build("pipeline_json", Box::new(pipeline_json_appender)))
        .logger(Logger::builder().build("stdout", level))
        .logger(
            Logger::builder()
                .appender("internal")
                .additive(false)
                .build("internal", level),
        )
        .logger(
            Logger::builder()
                .appender("pipeline_raw")
                .appender("pipeline_json")
                .build("pipeline", LevelFilter::Trace),
        )
        .build(
            Root::builder()
                .appender("stdout")
                .appender("internal")
                .build(level),
        )
        .unwrap();
    Ok(config)
}

/// Set loggers and return handler to change logLevels at runtime
pub fn set_logger(level: LevelFilter) -> Result<Handle, Box<dyn Error>> {
    let config = set_logger_config(level)?;
    // use handle to change logger configuration at runtime
    let handle = log4rs::init_config(config).unwrap();

    Ok(handle)
}
