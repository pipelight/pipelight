#![allow(dead_code)]

// Clap - command line lib
use clap::{Args, Parser, Subcommand};
use clap_verbosity_flag::Verbosity;

//Serde
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

#[derive(Debug, Clone, Parser)]
#[command(author, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,

    #[arg(long, global = true, hide = true)]
    /// Set a config file
    pub config: Option<String>,

    #[arg(global = true, long)]
    /// Attach command to standard I/O
    pub attach: bool,

    #[clap(flatten)]
    // #[serde(flatten)]
    /// Set verbosity level
    pub verbose: Verbosity,

    #[arg(global = true, last = true, allow_hyphen_values = true)]
    /// Pass those arguments to deno
    pub raw: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Subcommand)]
pub enum Commands {
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
    Watch,
}

#[derive(Debug, Clone, PartialEq, Parser)]
pub struct Pipeline {
    /// The pipeline name
    pub name: Option<String>,
    /// Attach command to standard I/O
    #[command(flatten)]
    pub trigger: Trigger,
}

#[derive(Debug, Clone, PartialEq, Parser)]
pub struct Trigger {
    /// Manualy set a flag/action to bypass environment computation
    #[arg(long, hide = true)]
    pub flag: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Parser)]
#[command(args_conflicts_with_subcommands = true)]
pub struct Logs {
    #[command(subcommand)]
    pub commands: Option<LogsCommands>,

    /// Display logs in json format
    #[command(flatten)]
    pub display: DisplayCommands,
}
#[derive(Debug, Clone, PartialEq, Parser)]
pub struct DisplayCommands {
    /// The pipeline name
    pub name: Option<String>,
    /// Display logs in json format
    #[arg(long)]
    pub json: bool,
}
#[derive(Debug, Clone, PartialEq, Parser)]
pub enum LogsCommands {
    /// Clear logs
    Rm,
}
