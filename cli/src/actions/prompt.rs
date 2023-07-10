use crate::actions::run;
use pipeline::{Getters, Node, Pipeline};

// Logging and verbosity
use log::LevelFilter;
use utils::git::Flag;
use utils::logger::logger;

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
            logger.lock().unwrap().level = LevelFilter::max();
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
