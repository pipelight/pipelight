use crate::actions::run;
use pipeline::{Getters, Node, Pipeline};

// Logging and verbosity
use log::LevelFilter;
use utils::logger::logger;

// Prompt
use dialoguer::{console::Term, theme::ColorfulTheme, Select};

// Error Handling
use miette::{IntoDiagnostic, Result};

pub fn inspect_prompt() -> Result<()> {
    let pipelines = Pipeline::get()?;
    let items = pipelines.iter().map(|e| &e.name).collect::<Vec<&String>>();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .into_diagnostic()?;

    match selection {
        Some(index) => {
            let node = Node::from(&pipelines[index]);
            logger.lock().unwrap().level = LevelFilter::max();
            println!("{}", node);
        }
        None => println!("User did not select anything"),
    }
    Ok(())
}
pub fn run_prompt(attach: bool) -> Result<()> {
    let pipelines = Pipeline::get()?;
    let items = pipelines.iter().map(|e| &e.name).collect::<Vec<&String>>();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .into_diagnostic()?;

    match selection {
        Some(index) => {
            let name = &pipelines[index].name;
            run::run_bin(name.to_owned(), attach)?;
        }
        None => println!("User did not select anything"),
    }
    Ok(())
}
