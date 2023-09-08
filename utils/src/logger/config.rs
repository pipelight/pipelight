pub use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};

use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Config, Logger, Root},
    encode::pattern::PatternEncoder,
};
use std::path::Path;

// use super::Logger;

// Error Handling
use miette::{Error, IntoDiagnostic, Result};

impl super::LogInfo {
    pub fn to_loggers(&mut self) -> Result<Vec<Logger>> {
        let loggers = vec![];
        let file_info = self.file_info;
        if let Some(file_info) = file_info {
            let path = format!(
                "{}/{}.json",
                file_info.directory.unwrap(),
                file_info.name.unwrap()
            );
            let appender = FileAppender::builder()
                .encoder(Box::new(PatternEncoder::new(&self.pattern)))
                .build(path)
                .unwrap();
            let logger = Logger::builder()
                .additive(false)
                .appender(format!("{}_to_file", self.name))
                .build(self.name, self.level);
            loggers.push(logger);
        }
        let appender = ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new(&self.pattern)))
            .build();
        let logger = Logger::builder()
            .additive(false)
            .appender(format!("{}_to_stdout", self.name))
            .build(self.name, self.level);
        loggers.push(logger);
        Ok(loggers)
    }
}

impl super::Logger {
    pub fn update(&mut self) -> Result<()> {
        let loggers = vec![];
        loggers.extend(self.pipelines.to_loggers()?);
        loggers.extend(self.internals.to_loggers()?);
        let config = Config::builder();
        for logger in loggers {
            config.logger(logger);
        }
        config.build(Root::builder().appender("stdout").build(LevelFilter::Trace));
        Ok(())
    }
}
