// Actions: Functions called by cli
use crate::types::*;
use log::{debug, error, info, trace, warn};

pub fn run(pipeline_name: String) {
    trace!("Run pipeline {} in the background", pipeline_name)
}
pub fn stop() {
    println!("config");
}
pub fn list() {
    println!("config");
}
pub fn logs() {
    println!("config");
}
