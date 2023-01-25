#![allow(unused_variables)]
#![allow(unused_must_use)]
#[allow(dead_code)]
use log::error;
use pipeline::types::Pipeline;
use std::env;
use std::error::Error;
use std::process::exit;

fn main() {
    handler().unwrap_or_else(|e| {
        error!("{}", e);
        exit(1)
    })
}

/// Launch detached subprocess
fn handler() -> Result<(), Box<dyn Error>> {
    run()?;
    Ok(())
}

/// Get command line args and run pipeline
pub fn run() -> Result<(), Box<dyn Error>> {
    // Collect Args
    let args = env::args().collect::<Vec<String>>();
    let pipeline_name: String = args[1].to_owned();

    let mut pipeline = Pipeline::name(&pipeline_name)?;
    pipeline.run();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn internal() {}
}
