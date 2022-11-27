// Cli
use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    ///Display a menu to easily select your pips
    #[arg(short, long)]
    interactiv: bool,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Manualy trigger a pipeline
    Pipe(Pipe),
    /// Display logs
    Logs(Logs),
}

#[derive(Debug, Parser)]
struct Pipe {
    #[arg(short, long)]
    /// Name of the pipeline to trigger
    trigger: String,

    #[arg(short, long)]
    /// Run in the backgroud (detach mode)
    detach: bool,

    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

#[derive(Parser, Debug)]
struct Logs {
    #[arg(short, long, action)]
    /// Filter logs with name of the git branch
    branch: bool,

    #[arg(short, long, action)]
    /// Filter logs with the name of the pipe
    pipe: bool,

    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}
pub fn get_args() {
    let args = Cli::parse();
}
