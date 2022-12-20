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
    /// Lint config file
    Lint(Empty),
    /// Enable hook functionnalities and
    /// Add some template config files if no config file detected
    Init(Empty),
}

#[derive(Debug, Parser)]
pub struct Empty {}

#[derive(Debug, Parser)]
/// The pipeline name
pub struct Pipeline {
    pub name: String,
}

#[derive(Parser, Debug)]
pub struct Logs {
    #[arg(short, long)]
    /// Display pretty logs
    pub pretty: bool,

    #[arg(short, long)]
    /// Display json logs
    pub json: bool,

    #[arg(short, long)]
    /// Display json logs
    pub clear: bool,

    #[arg(long, action, value_name = "BRANCH_NAME")]
    /// Filter logs on git branch (master,...)
    pub branch: Option<String>,

    #[arg(long, action, value_name = "GIT_ACTION")]
    /// Filter logs on git action (pre-push,...)
    pub action: Option<String>,

    #[arg(long, action, value_name = "PIPELINE_NAME")]
    /// Filter logs with the name of the pipe
    pub pipeline: Option<String>,

    #[clap(flatten)]
    pub verbose: Verbosity,
}
