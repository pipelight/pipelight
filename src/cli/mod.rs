// Cli
use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,

    ///Display a menu to easily select your pips
    #[arg(short, long)]
    interactiv: bool,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Manualy trigger a pipeline
    Pipe,
    /// Display logs
    Logs,
}

#[derive(Debug, Parser)]
pub struct Pipe {
    /// Name of the pipeline to trigger
    #[arg(short, long)]
    trigger: String,

    /// Run in the backgroud (detach mode)
    #[arg(short, long)]
    detach: bool,

    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

#[derive(Parser, Debug)]
pub struct Logs {
    /// Filter logs with name of the git branch
    #[arg(short, long, action)]
    branch: bool,

    /// Filter logs with the name of the pipe
    #[arg(short, long, action)]
    pipe: bool,

    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}
pub fn get_args() {
    let args = Cli::parse();
}
