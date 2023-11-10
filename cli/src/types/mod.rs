// Clap - command line lib
use clap::{Parser, Subcommand, ValueHint};
// Struct
pub use crate::verbosity::external::Verbosity;
pub use crate::verbosity::internal::InternalVerbosity;

// Struct
mod post;
mod pre;
pub use post::*;
pub use pre::*;

/*
The Cli struct is the entrypoint for command line argument parsing:
It casts arguments into the appropriate struct.

let args = Cli::from_arg_matches(&matches)

*/
#[derive(Debug, Clone, Parser)]
pub struct Cli {
    /**
    The set of subcommands.
    */
    #[command(subcommand)]
    pub commands: Commands,

    /**
    The folowing args are global arguments available
    for every subcommands.
    */
    /// Set a config file
    #[arg(long, global = true, hide = true, value_name="FILE" ,value_hint = ValueHint::FilePath)]
    pub config: Option<String>,

    /// Attach command to standard I/O
    #[arg(global = true, long)]
    pub attach: bool,

    /// Set verbosity level
    #[clap(flatten)]
    // #[serde(flatten)]
    pub verbose: Verbosity,

    #[clap(flatten)]
    // #[serde(flatten)]
    pub internal_verbose: InternalVerbosity,

    /// Pass those arguments to deno
    #[arg(global = true, last = true, allow_hyphen_values = true)]
    pub raw: Option<Vec<String>>,
}

/*
An enumaration over the differen types of commands available:
- PreCommand that only needs a partial env to run,
- PostCommands that needs the full env to be loaded to run.
*/
#[derive(Debug, Clone, Eq, PartialEq, Subcommand)]
pub enum Commands {
    #[clap(flatten)]
    PreCommands(PreCommands),
    #[clap(flatten)]
    PostCommands(PostCommands),
}
