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

impl super::Logger {
    pub fn update(&mut self) -> Result<()> {
        let mut loggers = vec![];
        loggers.extend(self.pipelines.make_loggers()?);
        loggers.extend(self.internals.make_loggers()?);
        let mut config = Config::builder();
        for logger in loggers {
            config = config.logger(logger);
        }
        let config = config
            .build(Root::builder().appender("stdout").build(LevelFilter::Trace))
            .into_diagnostic()?;
        let handle = log4rs::init_config(config).into_diagnostic()?;
        self.handle = Some(handle);
        Ok(())
    }
}
impl super::LogInfo {
    fn make_loggers(&mut self) -> Result<Vec<Logger>> {
        let mut loggers = vec![];
        let file_info = self.file_info.clone();
        if let Some(file_info) = file_info {
            // File
            let path = format!("{}/{}.json", file_info.directory, file_info.name);
            let name = format!("{}_to_file", self.name);
            let appender = FileAppender::builder()
                .encoder(Box::new(PatternEncoder::new(&self.pattern)))
                .build(path)
                .unwrap();
            let logger = Logger::builder()
                .additive(false)
                .appender(&name)
                .build(name, self.level);
            loggers.push(logger);
        }
        // Stdout
        let name = format!("{}_to_stdout", self.name);
        let appender = ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new(&self.pattern)))
            .build();
        let logger = Logger::builder()
            .additive(false)
            .appender(&name)
            .build(name, self.level);
        loggers.push(logger);
        // Nude
        let name = format!("{}_nude", self.name);
        let appender = ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{m}")))
            .build();
        let logger = Logger::builder()
            .additive(false)
            .appender(&name)
            .build(name, self.level);
        loggers.push(logger);
        Ok(loggers)
    }
}
