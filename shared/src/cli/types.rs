#![allow(dead_code)]

// Cli commands structure
use super::verbosity::Verbosity;
use clap::{Args, Parser, Subcommand};

//Serde
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

#[derive(Debug, Serialize, Deserialize, Clone, Parser)]
#[command(author, about, long_about = None)]
#[serde(deny_unknown_fields)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,

    #[arg(long, global = true, hide = true)]
    /// Set a config file
    pub config: Option<String>,

    #[clap(flatten)]
    // #[serde(flatten)]
    /// Set verbosity level
    pub verbose: Verbosity,

    #[arg(global = true, last = true, allow_hyphen_values = true)]
    /// Pass those arguments to deno
    pub raw: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Subcommand)]
#[serde(deny_unknown_fields)]
pub enum Commands {
    /// Get args as json
    Raw(Raw),
    /// Run a pipeline
    Run(Pipeline),
    /// Stop the pipeline execution (kill subprocess)
    #[command(arg_required_else_help = true)]
    Stop(Pipeline),
    /// Display logs
    Logs(Logs),
    /// List pipelines
    Ls(DisplayCommands),
    /// List pipelines (intercative)
    Inspect(DisplayCommands),
    /// Manualy Triggers Pipelines
    #[command(hide = true)]
    Trigger(Trigger),
    /// Launch a watcher on directory
    #[command(hide = true)]
    Watch(Watch),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Parser)]
#[serde(deny_unknown_fields)]
pub struct Empty {}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Parser)]
#[serde(deny_unknown_fields)]
pub struct Raw {
    /// The pipeline name
    pub string: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Parser)]
#[serde(deny_unknown_fields)]
pub struct Pipeline {
    /// The pipeline name
    pub name: Option<String>,
    /// Attach command to standard I/O
    #[command(flatten)]
    pub trigger: Trigger,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Parser)]
#[serde(deny_unknown_fields)]
pub struct Watch {
    /// Attach command to standard I/O
    #[arg(long)]
    pub attach: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Parser)]
#[serde(deny_unknown_fields)]
pub struct Trigger {
    /// Attach command to standard I/O
    #[arg(long)]
    pub attach: bool,
    /// Manualy set a flag/action to bypass environment computation
    #[arg(long, hide = true)]
    pub flag: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Parser)]
#[serde(deny_unknown_fields)]
#[command(args_conflicts_with_subcommands = true)]
pub struct List {
    /// Display logs in json format
    #[command(flatten)]
    pub display: DisplayCommands,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Parser)]
#[serde(deny_unknown_fields)]
#[command(args_conflicts_with_subcommands = true)]
pub struct Logs {
    #[command(subcommand)]
    pub commands: Option<LogsCommands>,

    /// Display logs in json format
    #[command(flatten)]
    pub display: DisplayCommands,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Parser)]
#[serde(deny_unknown_fields)]
pub struct DisplayCommands {
    /// The pipeline name
    pub name: Option<String>,
    /// Display logs in json format
    #[arg(long)]
    pub json: bool,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Parser)]
#[serde(deny_unknown_fields)]
pub enum LogsCommands {
    /// Clear logs
    Rm,
}
