// Clap - command line lib
//
// Error Handling
use miette::{IntoDiagnostic, Result};
// Standard I/O
use std::io;

pub use super::verbosity::external::Verbosity;
pub use super::verbosity::internal::InternalVerbosity;

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
    // #[command(hide = true)]
    Trigger(Trigger),
    /// Launch a watcher on directory
    // #[command(hide = true)]
    Watch(Watch),
    /// Generate autocompletion script
    Completion(Shell),
    /// Create a pipeline template file
    Init(Init),
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
pub struct Init;

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
    #[arg(long)]
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
    #[arg(long, default_missing_value = "always")]
    pub color: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Parser)]
pub enum LogsCommands {
    /// Clear logs
    Rm,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ColoredOutput {
    Always,
    Never,
}

#[derive(Debug, Clone, Eq, PartialEq, Parser)]
pub struct Shell {
    /// The shell name
    pub name: String,
}
/// Build the cli
impl Cli {
    pub fn build() -> Result<Command> {
        let mut cli = Command::new("pipelight");
        cli = Cli::augment_args(cli);
        cli = cli
            .mut_subcommand("completion", |a| {
                a.mut_arg("name", |e| {
                    e.value_parser([
                        PossibleValue::new("bash"),
                        PossibleValue::new("zsh"),
                        PossibleValue::new("fish"),
                        PossibleValue::new("elvish"),
                    ])
                })
            })
            .mut_arg("config", |e| e.value_hint(ValueHint::FilePath))
            .mut_subcommand("logs", |a| {
                a.mut_arg("color", |e| e.default_missing_value("always"))
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
