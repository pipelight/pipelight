//modules
pub mod traits;
pub mod types;
pub mod verbosity;

//Re-export
pub use types::{
    Cli,
    ColoredOutput,
    Commands,
    DisplayCommands,
    Init,
    InternalVerbosity,
    Logs,
    LogsCommands,
    Pipeline,
    // All types are implicitly exported
    // No globbing export like types::*
    // For better module readability
    Shell,
    Trigger,
    Verbosity,
    Watch,
    WatchCommands,
};
