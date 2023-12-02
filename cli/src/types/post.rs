// Clap - command line lib
use clap::{Parser, Subcommand};
// Struct
pub use crate::verbosity::external::Verbosity;
pub use crate::verbosity::internal::InternalVerbosity;
// Serde
use serde::{Deserialize, Serialize};

/*
Commands that need the config file to be found and loaded
Leads to a slowest execution time
*/
#[derive(Debug, Clone, Eq, PartialEq, Subcommand)]
pub enum PostCommands {
    #[clap(flatten)]
    DetachableCommands(DetachableCommands),
    /// Stop the pipeline execution and its every child processes
    Stop(Pipeline),
    /// Display pipelines logs
    Logs(Logs),
    /// List available pipelines with a few more useful informations
    Ls(DisplayCommands),
    /// Displays pipelines with the maximum verbosity level (interactive)
    Inspect(DisplayCommands),
}

/*
Commands that can be run in background
*/
#[derive(Debug, Clone, Eq, PartialEq, Subcommand)]
pub enum DetachableCommands {
    /// Run a pipeline (interactive)
    Run(Pipeline),
    /// Manualy trigger pipelines
    Trigger(Trigger),
    /// Launch a watcher on the working directory (debugging)
    #[command(hide = true)]
    Watch,
}

#[derive(Debug, Clone, Eq, PartialEq, Parser)]
#[command(args_conflicts_with_subcommands = true)]
pub struct Watch {
    // #[command(flatten)]
    // pub toggle: Option<Toggle>,
}

/**
Argument for pipeline execution.
- name: pipeline name,
- trigger: multiple triggering environment arguments.
*/
#[derive(Debug, Clone, Eq, PartialEq, Parser)]
pub struct Pipeline {
    /// The pipeline name
    pub name: Option<String>,
    #[command(flatten)]
    pub trigger: Trigger,
}

/**
Arguments to set/modify the triggering environment.
*/
#[derive(Debug, Clone, Eq, PartialEq, Parser)]
pub struct Trigger {
    /// Manualy set a flag/action to bypass environment computation.
    #[arg(long, ignore_case = true)]
    pub flag: Option<String>,
}
/**
Arguments to query logs.
*/
#[derive(Debug, Clone, Eq, PartialEq, Parser)]
#[command(args_conflicts_with_subcommands = true)]
pub struct Logs {
    #[command(subcommand)]
    pub commands: Option<LogsCommands>,

    /// Display logs in json format
    #[command(flatten)]
    pub display: DisplayCommands,
}

#[derive(Debug, Clone, Eq, PartialEq, Parser)]
pub enum LogsCommands {
    /// Clear logs
    Rm,
}

#[derive(Debug, Clone, Eq, PartialEq, Parser)]
pub struct DisplayCommands {
    /// The pipeline name
    pub name: Option<String>,

    /// Display logs in json format
    #[arg(long)]
    pub json: bool,

    /// Ignore the environment and enforce/disable colored output
    #[arg(long)]
    pub color: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ColoredOutput {
    Always,
    Auto,
    Never,
}
