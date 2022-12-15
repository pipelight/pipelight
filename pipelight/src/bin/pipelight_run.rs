#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
use log::LevelFilter::{Debug, Trace};
#[allow(dead_code)]
use shared::config::get_pipeline;
use shared::exec::shell;
use shared::logger::{debug, error, info, set_logger, trace, warn};
use std::env;
use std::error::Error;

/// Launch detached subprocess
fn main() -> Result<(), Box<dyn Error>> {
    set_logger(Trace);
    let args = env::args().collect::<Vec<String>>();
    let pipeline_name: String = args[1].to_owned();

    let pipeline = get_pipeline(pipeline_name);

    for step in pipeline.steps {
        for command in step.commands {
            let res = shell(command)?;
            println!("{}", res);
        }
    }
    Ok(())
}
