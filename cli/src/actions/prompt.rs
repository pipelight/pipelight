// Struct
use pipelight_exec::Status;
use workflow::{pipeline::Filters, Getters, Logs, Pipeline};
// Prompt
use dialoguer::{console::Term, Select};
// Error Handling
use miette::{Error, IntoDiagnostic, Result};

// Signal handling
use pipelight_utils::signal::restore_term;

/**
* Displays a select prompt of available pipelines
* and return the selected string
*/
pub fn pipeline() -> Result<String> {
    restore_term()?;

    // Get pipelines names
    let pipelines = Pipeline::get()?;
    let items = pipelines.iter().map(|e| &e.name).collect::<Vec<&String>>();

    // Displays a select prompt with pipeline names.
    let selection = Select::new()
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .into_diagnostic()?;

    // Return selected name
    match selection {
        Some(index) => {
            let name = &pipelines[index].name;
            Ok(name.to_owned())
        }
        None => {
            let message = "User did not select anything";
            Err(Error::msg(message))
        }
    }
}

/**
* Displays a select prompt of running pipelines
* and return the selected string
*/
pub fn running_pipeline() -> Result<String> {
    restore_term()?;

    // Get pipelines names
    let pipelines = Filters::filter_by_status(Logs::get()?, Some(Status::Running))?;
    let items = pipelines.iter().map(|e| &e.name).collect::<Vec<&String>>();
    // Guard
    if items.is_empty() {
        let message = "No running pipelines";
        return Err(Error::msg(message));
    }
    // Displays a select prompt with pipeline names.
    let selection = Select::new()
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .into_diagnostic()?;

    // Return selected name
    match selection {
        Some(index) => {
            let name = &pipelines[index].name;
            Ok(name.to_owned())
        }
        None => {
            let message = "User did not select anything";
            Err(Error::msg(message))
        }
    }
}
