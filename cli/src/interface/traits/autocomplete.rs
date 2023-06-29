use crate::{Cli, DisplayCommands, LogsCommands};
use clap::CommandFactory;
use clap_autocomplete;
// use clap_complete::{generate_to, shell::Zsh, shells::Bash};

// Error Handling
use miette::Result;

pub fn make_completion() -> Result<()> {
    let command = clap_autocomplete::add_subcommand(Cli::command());
    let command_copy = command.clone();
    let matches = command.get_matches();
    if let Some(result) = clap_autocomplete::test_subcommand(&matches, command_copy) {
        if let Err(err) = result {
            eprintln!("Insufficient permissions: {err}");
            std::process::exit(1);
        } else {
            std::process::exit(0);
        }
    } else { // Continue with the application logic
    }
    Ok(())
}
