// Clap - command line lib
use clap::{Parser, Subcommand, ValueHint};
// Struct
pub use crate::verbosity::external::Verbosity;
pub use crate::verbosity::internal::InternalVerbosity;
// Serde
use serde::{Deserialize, Serialize};

/*
Commands that does not need the config to be found and loaded.
Leads to fastest execution time.
*/
#[derive(Debug, Clone, Eq, PartialEq, Subcommand)]
pub enum PreCommands {
    /// Generate autocompletion script for most used shells (bash/zsh/fish)
    #[command(hide = true)]
    Completion(Shell),
    /// Create a `pipelight` config template file
    Init(Init),
    // Enable pipelight git hooks.
    #[command(arg_required_else_help = true)]
    Enable(Toggle),

    #[command(arg_required_else_help = true)]
    Disable(Toggle),
}

#[derive(Debug, Clone, Eq, PartialEq, Parser)]
#[command(args_conflicts_with_subcommands = true)]
pub struct Toggle {
    #[command(subcommand)]
    pub commands: Option<ToggleCommands>,
}

#[derive(Debug, Clone, Eq, PartialEq, Parser, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ToggleCommands {
    /// Git hooks toggle
    GitHooks,
    /// Watcher toggle
    Watcher,
}

/**
Arguments for initial config file template creation.
*/
#[derive(Debug, Clone, Eq, PartialEq, Parser)]
pub struct Init {
    /// The template style
    #[arg(long)]
    pub template: Option<String>,
    /// The output file path
    #[arg(long)]
    pub file: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Parser)]
pub struct Shell {
    /// The shell name
    #[arg(long, ignore_case = true)]
    pub name: String,
}
