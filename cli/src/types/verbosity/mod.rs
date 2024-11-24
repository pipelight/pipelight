pub mod external;
pub mod internal;

pub use external::Verbosity;
pub use internal::InternalVerbosity;

pub fn level_value(level: Option<log::Level>) -> i8 {
    match level {
        None => -1,
        Some(log::Level::Error) => 0,
        Some(log::Level::Warn) => 1,
        Some(log::Level::Info) => 2,
        Some(log::Level::Debug) => 3,
        Some(log::Level::Trace) => 4,
    }
}

fn level_enum(verbosity: i8) -> Option<log::Level> {
    match verbosity {
        std::i8::MIN..=-1 => None,
        0 => Some(log::Level::Error),
        1 => Some(log::Level::Warn),
        2 => Some(log::Level::Info),
        3 => Some(log::Level::Debug),
        4..=std::i8::MAX => Some(log::Level::Trace),
    }
}

pub trait LogLevel {
    fn default() -> Option<log::Level>;

    fn verbose_help() -> Option<&'static str> {
        Some("More output per occurrence")
    }

    fn verbose_long_help() -> Option<&'static str> {
        None
    }

    fn quiet_help() -> Option<&'static str> {
        Some("Less output per occurrence")
    }

    fn quiet_long_help() -> Option<&'static str> {
        None
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct ErrorLevel;

impl LogLevel for ErrorLevel {
    fn default() -> Option<log::Level> {
        Some(log::Level::Error)
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct WarnLevel;

impl LogLevel for WarnLevel {
    fn default() -> Option<log::Level> {
        Some(log::Level::Warn)
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct InfoLevel;

impl LogLevel for InfoLevel {
    fn default() -> Option<log::Level> {
        Some(log::Level::Info)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_app() {
        #[derive(Debug, clap::Parser)]
        struct Cli {
            #[clap(flatten)]
            verbose: InternalVerbosity,
        }

        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
