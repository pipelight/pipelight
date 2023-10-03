use crate::run;
// Struct
use cli::types::{Commands, DetachableCommands, PostCommands};
use workflow::{Getters, Logs, Node, Pipeline};
// Logging and verbosity
use log::LevelFilter;
use utils::globals::LOGGER;
// Prompt
use dialoguer::{console::Term, Select};
// Error Handling
use miette::{Error, IntoDiagnostic, Result};
// Globals
use cli::globals::CLI;

/**
Displays a selet prompt and add the selected pipeline name to the global CLI.
*/
pub fn run_prompt() -> Result<()> {
    // Displays a select prompt with pipeline names.
    let pipelines = Pipeline::get()?;
    let items = pipelines.iter().map(|e| &e.name).collect::<Vec<&String>>();
    let selection = Select::new()
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .into_diagnostic()?;

    // Add selected pipeline name to the CLI globals args.
    match selection {
        Some(index) => {
            let name = &pipelines[index].name;
            let mut args = CLI.lock().unwrap().clone();
            match args.commands {
                Commands::PostCommands(ref mut post) => match post {
                    PostCommands::DetachableCommands(detach) => match detach {
                        DetachableCommands::Run(pipeline) => pipeline.name = Some(name.to_owned()),
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            }
            *CLI.lock().unwrap() = args;
        }
        None => println!("User did not select anything"),
    }
    Ok(())
}

pub fn inspect_prompt() -> Result<()> {
    let pipelines = Pipeline::get()?;
    let items = pipelines.iter().map(|e| &e.name).collect::<Vec<&String>>();
    let selection = Select::new()
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .into_diagnostic()?;

    match selection {
        Some(index) => {
            LOGGER.lock().unwrap().pipelines.level = LevelFilter::max();
            let node = Node::from(&pipelines[index]);
            println!("{}", node);
        }
        None => println!("User did not select anything"),
    }
    Ok(())
}

pub fn stop_prompt() -> Result<()> {
    let pipelines: Vec<Pipeline> = Logs::get()?
        .into_iter()
        .filter(|e| e.is_running().is_ok())
        .collect();

    if pipelines.is_empty() {
        return Err(Error::msg("There is no running pipeline"));
    }

    let items = pipelines.iter().map(|e| &e.name).collect::<Vec<&String>>();
    let selection = Select::new()
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .into_diagnostic()?;

    match selection {
        Some(index) => {
            let pipeline = &mut pipelines[index].to_owned();
            pipeline.stop()?;
        }
        None => println!("User did not select anything"),
    }
    Ok(())
}
