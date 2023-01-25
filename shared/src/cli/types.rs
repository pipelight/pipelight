// Cli commands structure

use clap::{Args, Parser, Subcommand};
use clap_verbosity_flag::Verbosity;

#[derive(Debug, Parser)]
#[command(author, 
          //version,
           about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,

    #[clap(flatten)]
    /// Set verbosity level
    pub verbose: Verbosity,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Run a pipeline
    #[command(arg_required_else_help = true)]
    Run(Pipeline),
    /// Stop the pipeline execution (kill subprocess)
    #[command(arg_required_else_help = true)]
    Stop(Pipeline),
    /// Display logs
    Logs(Logs),
    /// List pipelines
    Ls(Empty),

    /// Manualy Triggers Pipelines
    #[command(hide = true)]
    Trigger(Empty),
}

#[derive(Debug, Parser)]
pub struct Empty {}

#[derive(Debug, Parser)]
/// The pipeline name
pub struct Pipeline {
    pub name: String,
}

#[derive(Debug, Subcommand)]
pub enum LogsCommands {
    /// Clear logs
    Rm(Empty),
}
#[derive(Debug, Args)]
pub struct DisplayCommands {
    /// Display logs in json format
    #[arg(long)]
    pub json: bool,
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct Logs {
    #[command(subcommand)]
    pub commands: Option<LogsCommands>,

    /// Display logs in json format
    #[command(flatten)]
    pub display: DisplayCommands,
}
