use crate::cli::actions::run;
use crate::workflow::{Getters, Logs, Node, Pipeline};

// Logging and verbosity
use crate::globals::LOGGER;
use log::LevelFilter;
use log::{error, info};

// Prompt
use dialoguer::{console::Term, Select};

// Error Handling
use miette::{Error, IntoDiagnostic, Result};

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
pub fn run_prompt(attach: bool, flag: Option<String>) -> Result<()> {
    let pipelines = Pipeline::get()?;
    let items = pipelines.iter().map(|e| &e.name).collect::<Vec<&String>>();
    let selection = Select::new()
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .into_diagnostic()?;

    match selection {
        Some(index) => {
            let name = &pipelines[index].name;
            run::launch(name.to_owned(), attach, flag)?;
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
