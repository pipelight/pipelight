use super::{level_enum, level_value, ErrorLevel, LogLevel};

#[derive(clap::Args, Debug, Clone, PartialEq)]
pub struct InternalVerbosity<L: LogLevel = ErrorLevel> {
    #[arg(
        long,
        short = 'u',
        action = clap::ArgAction::Count,
        global = true,
        help = L::verbose_help(),
        long_help = L::verbose_long_help(),
    )]
    internal_verbose: u8,

    #[arg(skip)]
    phantom: std::marker::PhantomData<L>,
}

impl<L: LogLevel> InternalVerbosity<L> {
    /// Create a new verbosity instance by explicitly setting the values
    pub fn new(verbose: u8, _quiet: u8) -> Self {
        InternalVerbosity {
            internal_verbose: verbose,
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
        level_value(L::default()) + (self.internal_verbose as i8)
    }
}

use std::fmt;

impl<L: LogLevel> fmt::Display for InternalVerbosity<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.verbosity())
    }
}
