// Actions: Functions called by cli
mod types;
use crate::shell::{exec_attach, exec_detach, load_config};
use crate::types::Config;
use log::{debug, error, info, trace, warn};
use std::error::Error;

pub fn run(pipeline_name: String) -> Result<(), Box<dyn Error>> {
    let config = load_config()?;
    trace!("Running pipeline {} in the background", pipeline_name);

    // Check duplicate
    let names = config
        .pipelines
        .iter()
        .map(|p| &p.name)
        .cloned()
        .collect::<Vec<String>>();

    //Vlone vector and emove duplicates
    let mut dedup = names.clone();
    dedup.dedup();

    //Compare bath vecors
    let has_duplicate = dedup.len() != names.len();

    trace!("{}", has_duplicate);
    debug!("{:?}", names);

    if has_duplicate {
        let message = "Duplicate pipeline names in config";
        error!("{}", message);
        Err(Box::from(message))
    } else {
        Ok(())
    }

    // for command in pipeline.
    // exec_attah(pi)
}

pub fn stop() {
    println!("config");
}

pub fn list() -> Result<(), Box<dyn Error>> {
    let config = load_config()?;
    // Print headers
    // String litteral might not be a variable (c injections issues)
    // let col = "{0: <10} {1: <20} {2: <10} {3}";
    println!(
        "{0: <10} {1: <20} {2: <10} {3}",
        "status", "last_run_date", "hook", "name"
    );
    for pipeline in config.pipelines {
        println!(
            "{0: <10} {1: <20} {2: <10} {3}",
            "status", "last_run date", "hook", pipeline.name
        )
    }
    Ok(())
}

pub fn logs() {
    println!("logs");
}
