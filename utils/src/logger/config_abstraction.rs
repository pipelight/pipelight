pub use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    Handle,
};
// Struct
use crate::logger::types::Logger;
// Error Handling
use miette::{IntoDiagnostic, Result};
// Globals
use crate::globals::LOGGER;

impl Logger {
    pub fn update(&mut self) -> Result<()> {
        let mut appenders = vec![];
        appenders.extend(self.pipelines.make_appenders()?);
        appenders.extend(self.internals.make_appenders()?);

        let mut loggers = vec![];
        loggers.extend(self.pipelines.make_loggers()?);
        loggers.extend(self.internals.make_loggers()?);

        let mut builder = Config::builder();
        for appender in appenders {
            builder = builder.appender(appender);
        }
        for logger in loggers {
            builder = builder.logger(logger);
        }
        let config: Config = if self.internals.file_info.is_some() {
            builder
                .build(
                    Root::builder()
                        .appender("internals_to_stdout")
                        .appender("internals_to_file")
                        .build(self.internals.level),
                )
                .into_diagnostic()?
        } else {
            builder
                .build(
                    Root::builder()
                        .appender("internals_to_stdout")
                        .build(self.internals.level),
                )
                .into_diagnostic()?
        };
        if self.handle.is_some() {
            self.reinit(config)?;
        } else {
            let handle = Logger::init(config)?;
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
                name,
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
            name,
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
            name,
            Box::new(
                ConsoleAppender::builder()
                    .encoder(Box::new(PatternEncoder::new("{m}")))
                    .build(),
            ),
        );
        appenders.push(appender);
        Ok(appenders)
    }
    fn make_loggers(&mut self) -> Result<Vec<log4rs::config::Logger>> {
        let mut loggers = vec![];
        let file_info = self.file_info.clone();
        if let Some(file_info) = file_info {
            // File
            let _path = format!("{}/{}.json", file_info.directory, file_info.name);
            let name = format!("{}_to_file", self.name);
            let logger = log4rs::config::Logger::builder()
                .additive(false)
                .appender(&name)
                .build(name, LevelFilter::Trace);
            loggers.push(logger);
        }
        // Stdout
        let name = format!("{}_to_stdout", self.name);
        let logger = log4rs::config::Logger::builder()
            .additive(false)
            .appender(&name)
            .build(name, self.level);
        loggers.push(logger);

        // Nude
        let name = format!("{}_nude", self.name);
        let logger = log4rs::config::Logger::builder()
            .additive(false)
            .appender(&name)
            .build(name, self.level);
        loggers.push(logger);
        Ok(loggers)
    }
}
