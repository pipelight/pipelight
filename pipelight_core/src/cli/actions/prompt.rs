use crate::cli::actions::run;
use crate::workflow::{Getters, Node, Pipeline};

// Logging and verbosity
use crate::globals::LOGGER;
use log::LevelFilter;

// Prompt
use dialoguer::{console::Term, Select};

// Error Handling
use miette::{IntoDiagnostic, Result};

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

pub fn stop_prompt(attach: bool, flag: Option<String>) -> Result<()> {
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
