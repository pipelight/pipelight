pub use log::Level::{Debug, Trace};
pub use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};

use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Config, Logger, Root},
    encode::pattern::PatternEncoder,
};
use std::path::Path;

use super::LoggerArgs;

/// Return logger config with chosen verbosity level and logging file "uuid.log"
pub fn default_set_file(args: LoggerArgs) -> Config {
    // Canonicalize paths
    let relative = args.pipelines.directory;
    let pipelines_dir = Path::new(&relative).canonicalize().unwrap();
    let relative = args.internals.directory;
    let internals_dir = Path::new(&relative).canonicalize().unwrap();

    // Set pipeline logs path.
    // let string = format!("{}/{}.json", args.pipelines.directory, args.pipelines.name);
    let string = format!("{}/{}.json", pipelines_dir.display(), args.pipelines.name);
    let logs_path = Path::new(&string);

    // let string = format!("{}/{}.txt", args.internals.directory, args.pipelines.name);
    let string = format!("{}/{}.txt", internals_dir.display(), args.pipelines.name);
    let internals_path = Path::new(&string);

    // Internal appenders
    let pattern = "{d(%Y-%m-%d %H:%M:%S)} | {h({l}):5.5} | {f}:{L} — {m}{n}";
    let stdout_appender = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build();
    let internal_appender = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build(internals_path.display().to_string())
        .unwrap();

    // Pipeline appender
    let json = "{m}{n}";
    let pipeline_json_appender = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(json)))
        .build(logs_path.display().to_string())
        .unwrap();
    let body = "{m}";
    let nude = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(body)))
        .build();

    Config::builder()
        .appender(Appender::builder().build("nude", Box::new(nude)))
        .appender(Appender::builder().build("pipeline_json", Box::new(pipeline_json_appender)))
        .appender(Appender::builder().build("stdout", Box::new(stdout_appender)))
        .appender(Appender::builder().build("internal", Box::new(internal_appender)))
        // Internals
        .logger(
            Logger::builder()
                .additive(false)
                .build("stdout", args.internals.level.to_owned()),
        )
        .logger(
            Logger::builder()
                .additive(false)
                .build("internal", args.internals.level.to_owned()),
        )
        // Pipelines
        .logger(
            Logger::builder()
                .additive(false)
                .appender("nude")
                .build("nude", args.pipelines.level.to_owned()),
        )
        .logger(
            Logger::builder()
                .additive(false)
                .appender("pipeline_json")
                .build("pipeline_json", LevelFilter::Trace),
        )
        .build(
            Root::builder()
                .appender("stdout")
                .build(args.internals.level.to_owned()),
        )
        .unwrap()
}

/// Return logger config with chosen verbosity level
pub fn default(args: LoggerArgs) -> Config {
    // Pipeline appender
    let body = "{m}";
    let nude = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(body)))
        .build();

    // Internal appenders
    let pattern = "{d(%Y-%m-%d %H:%M:%S)} | {h({l}):5.5} | {f}:{L} — {m}{n}";
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build();

    Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("nude", Box::new(nude)))
        .logger(
            Logger::builder()
                .additive(false)
                .appender("nude")
                .build("nude", args.internals.level.to_owned()),
        )
        .build(
            Root::builder()
                .appender("stdout")
                .build(args.internals.level.to_owned()),
        )
        .unwrap()
}
