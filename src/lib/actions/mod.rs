// Actions: Functions called by cli

// use crate::types::{Config, Pipeline};
use log::{debug, error, info, trace, warn};

pub fn run(pipeline_name: String) {
    trace!("Run pipeline {} in the background", pipeline_name)
}
pub fn stop() {}
pub fn list() {}
pub fn logs() {}

fn main() {}
