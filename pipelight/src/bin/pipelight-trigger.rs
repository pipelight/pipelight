#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
use log::LevelFilter::{Debug, Trace};
#[allow(dead_code)]
use shared::config::{get_config, get_pipeline};
use shared::logger::{debug, error, info, set_logger, trace, warn};
use shared::trigger::trigger;
use shared::types::config::Pipeline;
use shared::types::logs::{PipelineLog, PipelineStatus, StepLog};
use std::env;
use std::error::Error;
use std::path::Path;
use std::process::exit;

fn main() {
    handler().unwrap_or_else(|a| exit(1))
}
/// Launch attached subprocess
fn handler() -> Result<(), Box<dyn Error>> {
    trigger()?;
    Ok(())
}
