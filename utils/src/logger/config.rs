pub use log::Level::{Debug, Trace};
pub use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};
use log4rs;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use uuid::Uuid;

/// Return logger config with chosen verbosity level and logging file "uuid.log"
pub fn default_with_file(level: &LevelFilter, uuid: &Uuid) -> Config {
    let level = level.to_owned();
    let pattern = "{d(%Y-%m-%d %H:%M:%S)} | {h({l}):5.5} | {f}:{L} — {m}{n}";
    let json = "{m}{n}";
    let body = "{m}";
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build();
    let nude = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(body)))
        .build();
    let pipeline_json_appender = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(json)))
        .build(format!(".pipelight/logs/{}.json", uuid))
        .unwrap();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("nude", Box::new(nude)))
        .appender(Appender::builder().build("pipeline_json", Box::new(pipeline_json_appender)))
        .logger(Logger::builder().additive(false).build("stdout", level))
        .logger(
            Logger::builder()
                .additive(false)
                .appender("nude")
                .build("nude", level),
        )
        .logger(
            Logger::builder()
                .additive(false)
                .appender("pipeline_json")
                .build("pipeline_json", LevelFilter::Trace),
        )
        .build(Root::builder().appender("stdout").build(level))
        .unwrap();
    return config;
}

/// Return logger config with chosen verbosity level
pub fn default(level: &LevelFilter) -> Config {
    // let level = LevelFilter::Trace;
    let level = level.to_owned();
    let pattern = "{d(%Y-%m-%d %H:%M:%S)} | {h({l}):5.5} | {f}:{L} — {m}{n}";
    let body = "{m}";
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build();
    let nude = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(body)))
        .build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("nude", Box::new(nude)))
        .logger(Logger::builder().additive(false).build("stdout", level))
        .logger(
            Logger::builder()
                .additive(false)
                .appender("nude")
                .build("nude", level),
        )
        .build(Root::builder().appender("stdout").build(level))
        .unwrap();
    return config;
}
