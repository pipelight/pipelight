#![allow(unused_variables)]
#![allow(unused_must_use)]
use log::{debug, error, info, trace, warn};
use shared::logger::Logs;
use shared::types::logs::pipeline;
#[allow(dead_code)]
use std::env;
use std::error::Error;
use std::process::exit;

fn main() {
    handler().unwrap_or_else(|e| {
        // error!("{}", e);
        exit(1)
    })
}

/// Launch detached subprocess
fn handler() -> Result<(), Box<dyn Error>> {
    shared::run::run_bin()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn internal() {}
}
