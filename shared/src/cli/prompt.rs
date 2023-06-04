use crate::run;
use pipeline::types::{traits::getters::Getters, Node, Pipeline};

// Logging and verbosity
use log::{debug, error, info, warn, LevelFilter};
use utils::logger::logger;

// Prompt
use dialoguer::{console::Term, theme::ColorfulTheme, Select};
// Error Handling
use std::error::Error;

pub fn inspect_prompt() -> Result<(), Box<dyn Error>> {
    let pipelines = Pipeline::get()?;
    let items = pipelines.iter().map(|e| &e.name).collect::<Vec<&String>>();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

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
pub fn run_prompt(args: Option<Vec<String>>) -> Result<(), Box<dyn Error>> {
    let pipelines = Pipeline::get()?;
    let items = pipelines.iter().map(|e| &e.name).collect::<Vec<&String>>();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => {
            let name = &pipelines[index].name;
            run::run_bin(name.to_owned(), false, args)?;
        }
        None => println!("User did not select anything"),
    }
    Ok(())
}
