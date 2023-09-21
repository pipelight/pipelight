// Clap - command line lib
//
// Error Handling
use miette::{IntoDiagnostic, Result};
// Standard I/O
use std::io;

pub use super::verbosity::external::Verbosity;
pub use super::verbosity::internal::InternalVerbosity;

// serde
use serde::{Deserialize, Serialize};

use clap::{builder::PossibleValue, Args, Command, FromArgMatches, Parser, Subcommand, ValueHint};
use clap_complete::{generate, shells};

// Globals
pub use crate::globals::CLI;

#[derive(Debug, Clone, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,

    /// Set a config file
    #[arg(long, global = true, hide = true, value_name="FILE" ,value_hint = ValueHint::FilePath)]
    pub config: Option<String>,

    #[arg(global = true, long)]
    /// Attach command to standard I/O
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

#[derive(Debug, Clone, Eq, PartialEq, Subcommand)]
pub enum Commands {
    #[clap(flatten)]
    PreCommands(PreCommands),
    #[clap(flatten)]
    PostCommands(PostCommands),
}
#[derive(Debug, Clone, Eq, PartialEq, Subcommand)]
pub enum PreCommands {
    /// Generate autocompletion script for most used shells (bash/zsh/fish)
    #[command(hide = true)]
    Completion(Shell),
    /// Create a `pipelight` config template file
    Init(Init),
    /// Enable pipelight git hooks.
    #[command(arg_required_else_help = true)]
    Hooks(Toggle),
}

#[derive(Debug, Clone, Eq, PartialEq, Subcommand)]
pub enum PostCommands {
    /// Run a pipeline (interactive)
    Run(Pipeline),
    /// Stop the pipeline execution and its every child processes
    Stop(Pipeline),
    /// Display pipelines logs
    Logs(Logs),
    /// List available pipelines with a few more useful informations
    Ls(DisplayCommands),
    /// Displays pipelines with the maximum verbosity level (interactive)
    Inspect(DisplayCommands),
    /// Manualy trigger pipelines
    Trigger(Trigger),
    /// Launch a watcher on the working directory
    #[command(hide = true)]
    Watch(Watch),
}

#[derive(Debug, Clone, Eq, PartialEq, Parser)]
#[command(args_conflicts_with_subcommands = true)]
pub struct Watch {
    #[command(subcommand)]
    pub commands: Option<WatchCommands>,
}
#[derive(Debug, Clone, Eq, PartialEq, Parser)]
pub enum WatchCommands {
    Kill,
}
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
pub struct Toggle {
    /// Create git hooks
    #[arg(long, conflicts_with = "disable")]
    pub enable: bool,
    /// Remove git hooks
    #[arg(long, conflicts_with = "enable")]
    pub disable: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Parser)]
pub struct Pipeline {
    /// The pipeline name
    pub name: Option<String>,
    /// Attach command to standard I/O
    #[command(flatten)]
    pub trigger: Trigger,
}

#[derive(Debug, Clone, Eq, PartialEq, Parser)]
pub struct Trigger {
    /// Manualy set a flag/action to bypass environment computation
    #[arg(long, ignore_case = true)]
    pub flag: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Parser)]
#[command(args_conflicts_with_subcommands = true)]
pub struct Logs {
    #[command(subcommand)]
    pub commands: Option<LogsCommands>,

    /// Display logs in json format
    #[command(flatten)]
    pub display: DisplayCommands,
}

#[derive(Debug, Clone, Eq, PartialEq, Parser)]
pub struct DisplayCommands {
    /// The pipeline name
    pub name: Option<String>,

    /// Display logs in json format
    #[arg(long)]
    pub json: bool,

    /// Ignore the environment and enforce/disable colored output
    #[arg(long)]
    pub color: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Parser)]
pub enum LogsCommands {
    /// Clear logs
    Rm,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ColoredOutput {
    Always,
    Auto,
    Never,
}

#[derive(Debug, Clone, Eq, PartialEq, Parser)]
pub struct Shell {
    /// The shell name
    #[arg(long, ignore_case = true)]
    pub name: String,
}
/// Build the cli
impl Cli {
    pub fn build() -> Result<Command> {
        let mut cli = Command::new("pipelight");
        cli = Cli::augment_args(cli);
        cli = cli
            .mut_subcommand("logs", |a| {
                a.mut_arg("color", |e| {
                    e.num_args(0..=1)
                        .require_equals(true)
                        .default_missing_value("always")
                        .default_value("auto")
                })
            })
            .mut_arg("config", |e| e.value_hint(ValueHint::FilePath))
            .mut_subcommand("completion", |a| {
                a.mut_arg("name", |e| {
                    e.value_parser([
                        PossibleValue::new("bash"),
                        PossibleValue::new("zsh"),
                        PossibleValue::new("fish"),
                        PossibleValue::new("elvish"),
                    ])
                })
            });
        Ok(cli)
    }
    pub fn print_completion(shell: shells::Shell) -> Result<()> {
        // Build client and generate autocompletion script
        let mut cmd = Cli::build()?;
        let name = cmd.get_name().to_string();
        generate(shell, &mut cmd, name, &mut io::stdout());

        Ok(())
    }
    // Hydrate cli
    pub fn hydrate_global() -> Result<()> {
        let cli = Cli::build()?;
        let matches = cli.get_matches();
        let args = Cli::from_arg_matches(&matches)
            .map_err(|err| err.exit())
            .unwrap();
        unsafe { *CLI = args.clone() };
        Ok(())
    }
}
