// #![allow(unused_variables)]
// #![allow(unused_must_use)]
// #![allow(unused_imports)]
// #![allow(dead_code)]

pub mod actions;
pub mod case;
pub mod interface;

// Cli test
pub mod test;

//Re-export
pub use case::CLI;
pub use interface::{
    // Every types are implicitly exported
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
