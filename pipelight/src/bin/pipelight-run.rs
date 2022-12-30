#![allow(unused_variables)]
#![allow(unused_must_use)]
use log::{debug, error, info, trace, warn};
use shared::logger::Logs;
use shared::types::logs::PipelineLog;
#[allow(dead_code)]
use shared::types::Config;
use shared::types::Pipeline;
use std::env;
use std::error::Error;
use std::process::exit;

fn main() {
    handler().unwrap_or_else(|e| {
        // error!("{}", e);
        exit(1)
    })
}

/// Launch attached subprocess
fn handler() -> Result<(), Box<dyn Error>> {
    // Collect Args
    let args = env::args().collect::<Vec<String>>();
    let pipeline_name: String = args[1].to_owned();

    let p: Pipeline = Config::new()?.pipeline(&pipeline_name)?;
    let mut pipeline = PipelineLog::from(&p);

    let handle = Logs::new().set()?;
    pipeline.run(&handle);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::type_of;
    #[test]
    fn internal() {
        let res = load_config();
    }
}
