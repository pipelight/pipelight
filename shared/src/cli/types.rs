// Cli commands structure

use clap::{Args, Parser, Subcommand, ValueEnum};
use clap_verbosity_flag::{InfoLevel, Verbosity};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[clap(flatten)]
    /// Set verbosity level
    verbose: Verbosity<InfoLevel>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Run a pipeline
    Run(Pipeline),
    /// Stop the pipeline execution (kill subprocess)
    Stop(Pipeline),
    /// Display logs
    Logs(Logs),
    /// List pipelines
    Ls(List),
}

#[derive(Debug, Parser)]
pub struct List {}

#[derive(Debug, Parser)]
/// The pipeline name
pub struct Pipeline {
    pub name: String,
}

#[derive(Parser, Debug)]
pub struct Logs {
    #[arg(short, long, action)]
    /// Display pretty logs
    pretty: Option<bool>,

    #[arg(long, action, value_name = "BRANCH_NAME")]
    /// Filter logs on git branch (master,...)
    branch: Option<String>,

    #[arg(long, action, value_name = "GIT_ACTION")]
    /// Filter logs on git action (pre-push,...)
    action: Option<String>,

    #[arg(long, action, value_name = "PIPELINE_NAME")]
    /// Filter logs with the name of the pipe
    pipeline: Option<String>,

    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}
