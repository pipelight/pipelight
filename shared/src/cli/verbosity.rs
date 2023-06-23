// Serialized version of clap verbosity flag
// https://docs.rs/clap-verbosity-flag/latest/src/clap_verbosity_flag/lib.rs.html#62-86

pub use log::Level;
pub use log::LevelFilter;

//Serde
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

#[derive(clap::Args, Serialize, Deserialize, Debug, Clone)]
pub struct Verbosity<L: LogLevel = ErrorLevel> {
    #[arg(
        long,
        short = 'v',
        action = clap::ArgAction::Count,
        global = true,
        help = L::verbose_help(),
        long_help = L::verbose_long_help(),
    )]
    verbose: u8,

    #[arg(
        long,
        short = 'q',
        action = clap::ArgAction::Count,
        global = true,
        help = L::quiet_help(),
        long_help = L::quiet_long_help(),
        conflicts_with = "verbose",
    )]
    quiet: u8,

    #[arg(skip)]
    phantom: std::marker::PhantomData<L>,
}

impl<L: LogLevel> Verbosity<L> {
    /// Create a new verbosity instance by explicitly setting the values
    pub fn new(verbose: u8, quiet: u8) -> Self {
        Verbosity {
            verbose,
            quiet,
            phantom: std::marker::PhantomData,
        }
    }

    /// Get the log level.
    ///
    /// `None` means all output is disabled.
    pub fn log_level(&self) -> Option<log::Level> {
        level_enum(self.verbosity())
    }

    /// Get the log level filter.
    pub fn log_level_filter(&self) -> log::LevelFilter {
        level_enum(self.verbosity())
            .map(|l| l.to_level_filter())
            .unwrap_or(log::LevelFilter::Off)
    }

    /// If the user requested complete silence (i.e. not just no-logging).
    pub fn is_silent(&self) -> bool {
        self.log_level().is_none()
    }

    fn verbosity(&self) -> i8 {
        level_value(L::default()) - (self.quiet as i8) + (self.verbose as i8)
    }
}

fn level_value(level: Option<log::Level>) -> i8 {
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

// use std::fmt;

// impl<L: LogLevel> fmt::Display for Verbosity<L> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.verbosity())
//     }
// }

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

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Default)]
pub struct ErrorLevel;

impl LogLevel for ErrorLevel {
    fn default() -> Option<log::Level> {
        Some(log::Level::Error)
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Default)]
pub struct WarnLevel;

impl LogLevel for WarnLevel {
    fn default() -> Option<log::Level> {
        Some(log::Level::Warn)
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Default)]
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
        #[derive(Serialize, Deserialize, Debug, clap::Parser)]
        struct Cli {
            #[clap(flatten)]
            verbose: Verbosity,
        }

        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
