pub use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};

use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Config, Logger, Root},
    encode::pattern::PatternEncoder,
    Handle,
};
use std::path::Path;

// use super::Logger;

// Error Handling
use miette::{Error, IntoDiagnostic, Result};

impl super::Logger {
    pub fn update(&mut self) -> Result<()> {
        let mut appenders = vec![];
        appenders.extend(self.pipelines.make_appenders()?);
        appenders.extend(self.internals.make_appenders()?);

        let mut loggers = vec![];
        loggers.extend(self.pipelines.make_loggers()?);
        loggers.extend(self.internals.make_loggers()?);

        let mut config = Config::builder();
        for appender in appenders {
            config = config.appender(appender);
        }
        for logger in loggers {
            config = config.logger(logger);
        }
        let config = config
            .build(Root::builder().build(LevelFilter::Trace))
            .into_diagnostic()?;
        if self.handle.is_some() {
            self.reinit(config);
        } else {
            let handle = super::Logger::init(config)?;
            self.handle = Some(handle);
        }
        Ok(())
    }
    pub fn init(config: Config) -> Result<Handle> {
        let handle = log4rs::init_config(config).into_diagnostic()?;
        Ok(handle)
    }
    pub fn reinit(&mut self, config: Config) -> Result<()> {
        self.handle.as_mut().unwrap().set_config(config);
        Ok(())
    }
}

impl super::LogInfo {
    fn make_appenders(&mut self) -> Result<Vec<Appender>> {
        let mut appenders = vec![];
        let file_info = self.file_info.clone();
        if let Some(file_info) = file_info {
            // File
            let path = format!("{}/{}.json", file_info.directory, file_info.name);
            let name = format!("{}_to_file", self.name);
            let appender = Appender::builder().build(
                &name,
                Box::new(
                    FileAppender::builder()
                        .encoder(Box::new(PatternEncoder::new(&self.pattern)))
                        .build(path)
                        .unwrap(),
                ),
            );
            appenders.push(appender);
        }
        // Stdout
        let name = format!("{}_to_stdout", self.name);
        let appender = Appender::builder().build(
            &name,
            Box::new(
                ConsoleAppender::builder()
                    .encoder(Box::new(PatternEncoder::new(&self.pattern)))
                    .build(),
            ),
        );
        appenders.push(appender);
        // Nude
        let name = format!("{}_nude", self.name);
        let appender = Appender::builder().build(
            &name,
            Box::new(
                ConsoleAppender::builder()
                    .encoder(Box::new(PatternEncoder::new("{m}")))
                    .build(),
            ),
        );
        appenders.push(appender);
        Ok(appenders)
    }
    fn make_loggers(&mut self) -> Result<Vec<Logger>> {
        let mut loggers = vec![];
        let file_info = self.file_info.clone();
        if let Some(file_info) = file_info {
            // File
            let path = format!("{}/{}.json", file_info.directory, file_info.name);
            let name = format!("{}_to_file", self.name);
            let logger = Logger::builder()
                .additive(false)
                .appender(&name)
                .build(name, self.level);
            loggers.push(logger);
        }
        // Stdout
        let name = format!("{}_to_stdout", self.name);
        let logger = Logger::builder()
            .additive(false)
            .appender(&name)
            .build(name, self.level);
        loggers.push(logger);

        // Nude
        let name = format!("{}_nude", self.name);
        let logger = Logger::builder()
            .additive(false)
            .appender(&name)
            .build(name, self.level);
        loggers.push(logger);
        Ok(loggers)
    }
}
