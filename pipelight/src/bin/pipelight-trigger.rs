#![allow(unused_variables)]
#[allow(dead_code)]
use shared::trigger::trigger;
use std::error::Error;
use std::process::exit;

fn main() {
    handler().unwrap_or_else(|a| exit(1))
}
/// Launch attached subprocess
fn handler() -> Result<(), Box<dyn Error>> {
    trigger()?;
    Ok(())
}
