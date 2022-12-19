#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
use log::LevelFilter::{Debug, Trace};
#[allow(dead_code)]
use shared::config::get_pipeline;
use shared::logger::{debug, error, info, set_logger, trace, warn};
use shared::types::logs::{PipelineLog, PipelineStatus, StepLog};
use shared::types::Pipeline;
use std::env;
use std::error::Error;
use std::process::exit;

fn main() {
    handler().unwrap_or_else(|a| exit(1))
}
/// Launch attached subprocess
fn handler() -> Result<(), Box<dyn Error>> {
    set_logger(Trace);
    Ok(())
}
