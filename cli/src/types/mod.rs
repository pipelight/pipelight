// Traits implementation
pub mod display;
pub mod from_string;

// Clap - command line lib
use clap::{Parser, Subcommand, ValueHint};
// Verbosity
pub mod verbosity;
pub use verbosity::{InternalVerbosity, Verbosity};
// Serde
use serde::{Deserialize, Serialize};

/*
The Cli struct is the entrypoint for command line argument parsing:
It casts arguments into the appropriate struct.

let args = Cli::from_arg_matches(&matches)

*/
#[derive(Debug, Clone, Parser, PartialEq)]
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
    #[arg(global = true, long, required = false)]
    pub attach: Option<String>,

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
impl Default for Cli {
    fn default() -> Self {
        Cli {
            commands: Commands::PostCommands(PostCommands::Ls(DisplayCommands {
                json: false,
                name: None,
                color: None,
            })),
            raw: None,
            config: None,
            verbose: Verbosity::new(0, 0),
            internal_verbose: InternalVerbosity::new(0, 0),
            attach: None,
        }
    }
}

/*
Why this and not a simple boolean?
Clap interprets Option<bool> as bool.
This enum is a workaround.
It can be either None, Some("true") or Some("false")

Then it is possible to know if the flag has been used
on command line.

*/
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Attach {
    True,
    False,
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

    // Get the executable version number
    Version,
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

/*
Commands that need the config file to be found and loaded
Leads to a slowest execution time
*/
#[derive(Debug, Clone, Eq, PartialEq, Subcommand)]
pub enum PostCommands {
    #[clap(flatten)]
    DetachableCommands(DetachableCommands),
    /// Stop the pipeline execution and its every child processes
    Stop(Pipeline),
    /// Display pipelines logs
    Logs(Logs),
    /// List available pipelines with a few more useful informations
    Ls(DisplayCommands),
    /// Displays pipelines with the maximum verbosity level (interactive)
    Inspect(DisplayCommands),
}

/*
Commands that can be run in background
*/
#[derive(Debug, Clone, Eq, PartialEq, Subcommand)]
pub enum DetachableCommands {
    /// Run a pipeline (interactive)
    Run(Pipeline),
    /// Manualy trigger pipelines
    Trigger(Trigger),
    /// Launch a watcher on the working directory (debugging)
    #[command(hide = true)]
    Watch,
}

#[derive(Debug, Clone, Eq, PartialEq, Parser)]
#[command(args_conflicts_with_subcommands = true)]
pub struct Watch {
    // #[command(flatten)]
    // pub toggle: Option<Toggle>,
}

/**
Argument for pipeline execution.
- name: pipeline name,
- trigger: multiple triggering environment arguments.
*/
#[derive(Debug, Clone, Eq, PartialEq, Parser)]
pub struct Pipeline {
    /// The pipeline name
    pub name: Option<String>,
    #[command(flatten)]
    pub trigger: Trigger,
}
impl Default for Pipeline {
    fn default() -> Self {
        Pipeline {
            name: Some("default".to_owned()),
            trigger: Trigger {
                flag: Some("blank".to_owned()),
            },
        }
    }
}

/**
Arguments to set/modify the triggering environment.
*/
#[derive(Debug, Clone, Eq, PartialEq, Parser)]
pub struct Trigger {
    /// Manualy set a flag/action to bypass environment computation.
    #[arg(long, ignore_case = true)]
    pub flag: Option<String>,
}
/**
Arguments to query logs.
*/
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
pub enum LogsCommands {
    /// Clear logs
    Rm,
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

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ColoredOutput {
    Always,
    Auto,
    Never,
}
