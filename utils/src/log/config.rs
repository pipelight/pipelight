pub use log::Level::{Debug, Trace};
pub use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};
use log4rs;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use uuid::Uuid;

pub fn file(level: LevelFilter, uuid: Uuid) -> Config {
    // The raw logger will be implemented if needed
    // to troubleshoot simultaneous pipline execution
    // with same file acces error...

    // let shell_pattern = "{d(%Y-%m-%d %H:%M:%S)} | {h({l}):5.5} | {f}:{L} — \n{m}{n}\n";
    let pattern = "{d(%Y-%m-%d %H:%M:%S)} | {h({l}):5.5} | {f}:{L} — {m}{n}";
    let json = "{m}{n}";
    let body = "{m}";
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build();
    let nude = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(body)))
        .build();
    // let pipeline_raw_appender = FileAppender::builder()
    //     .encoder(Box::new(PatternEncoder::new(shell_pattern)))
    //     .build(format!(".pipelight/logs/{}.raw", uuid))
    //     .unwrap();
    let pipeline_json_appender = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(json)))
        .build(format!(".pipelight/logs/{}.json", uuid))
        .unwrap();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("nude", Box::new(nude)))
        // .appender(Appender::builder().build("pipeline_raw", Box::new(pipeline_raw_appender)))
        .appender(Appender::builder().build("pipeline_json", Box::new(pipeline_json_appender)))
        .logger(Logger::builder().additive(false).build("stdout", level))
        .logger(
            Logger::builder()
                .additive(false)
                .appender("nude")
                .build("nude", level),
        )
        // .logger(
        //     Logger::builder()
        //         .additive(false)
        //         .appender("pipeline_raw")
        //         .build("pipeline_raw", LevelFilter::Trace),
        // )
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
pub fn default(level: LevelFilter) -> Config {
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
