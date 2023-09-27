// #![allow(unused_variables)]
// #![allow(unused_must_use)]
// #![allow(unused_imports)]
// #![allow(dead_code)]

pub mod actions;
pub mod case;
pub mod globals;
pub mod interface;
pub mod utils;

// Cli test
pub mod test;

pub use interface::{
    // Every types are implicitly exported
    Cli,
    ColoredOutput,
    Commands,
    DisplayCommands,
    Init,
    Logs,
    LogsCommands,
    Pipeline,
    Trigger,
    Watch,
    WatchCommands,
};
