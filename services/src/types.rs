// Struct
use actions::types::Action;
use cli::types::Cli;
// Watchexec
use watchexec::{
    config::{InitConfig, RuntimeConfig},
    handler::{Handler as _, PrintDebug},
    Watchexec,
};

#[derive(Debug, Clone)]
pub struct Service {
    pub args: Option<Cli>,
    pub action: Action,
}

#[derive(Debug)]
pub struct WatchExec {
    watchexec: Option<Watchexec>,
    runtime: Option<RuntimeConfig>,
    builded: bool,
}
