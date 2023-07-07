//modules
pub mod traits;
pub mod types;

//Re-export
pub use types::{
    // All types are implicitly exported
    // No globbing export like types::*
    // For better module readability
    Cli,
    ColoredOutput,
    Commands,
    DisplayCommands,
    Logs,
    LogsCommands,
    Pipeline,
    Trigger,
    Watch,
    WatchCommands,
};
