// Actions: Functions called by cli
mod types;
use crate::types::Config;
use log::{debug, error, info, trace, warn};

pub fn run(pipeline_name: String) {
    trace!("Run pipeline {} in the background", pipeline_name)
}
pub fn stop() {
    println!("config");
}
pub fn list(config: Config) {
    println!(
        "{0: <10} | {1: <10} | {2: <10} | {3: <10}",
        "status", "last_run date", "hook", "name"
    );

    for pipeline in config.pipelines {
        for step in pipeline.steps {
            println!("{}", step.name)
        }
    }

    // println!("{:#?}", config)
}
pub fn logs() {
    println!("config");
}
