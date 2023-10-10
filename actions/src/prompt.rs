// Struct
use workflow::{Getters, Pipeline};
// Prompt
use dialoguer::{console::Term, Select};
// Error Handling
use miette::{Error, IntoDiagnostic, Result};

/**
Displays a selet prompt and add the selected pipeline name to the global CLI.
*/
pub fn pipeline() -> Result<String> {
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
