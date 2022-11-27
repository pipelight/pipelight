// Cli
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Name of the pipeline to trigger
    #[arg(short, long)]
    pipeline: String,

    /// Display logs
    #[arg(short, long)]
    logs: String,

    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}
