#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#[allow(dead_code)]
use shared::config::get_config;
use shared::exec::exec_attach;
use shared::logger::{debug, error, info, trace, warn};
use std::env;
use std::error::Error;

/// Launch detached subprocess

fn main() -> Result<(), Box<dyn Error>> {
    let config = get_config()?;

    let args = env::args().collect::<Vec<String>>();
    let pipeline_name: String = args[1].to_owned();

    let pipeline_result = config
        .pipelines
        .iter()
        .filter(|p| p.name == pipeline_name)
        .cloned()
        .next();

    if pipeline_result.is_some() {
        let pipeline = pipeline_result.unwrap();
        trace!("Running pipeline {} in the background", pipeline.name);
        let output = exec_attach(format!("neo"))?;
        println!("{:#?}", output);
        Ok(())
    } else {
        let message = "Pipeline doesn't exist";
        error!("{}", message);
        Err(Box::from(message))
    }
    // for step in pipeline.steps {
    //     for command in step.commands{
    //         exec_attach(command);
    //     }
    // }
}
