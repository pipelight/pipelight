#![allow(unused_variables)]
#![allow(unused_must_use)]
use log::error;
use pipeline::types::Config;
use shared::trigger::trigger;
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
    let mut args = env::args().collect::<Vec<String>>();
    args.remove(0);
    let pipeline_name: String = args[0].to_owned();
    args.remove(0);

    Config::new(Some(args));

    // Detached process
    trigger(false)?;
    Ok(())
}
