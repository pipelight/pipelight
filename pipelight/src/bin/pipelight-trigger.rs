#![allow(unused_variables)]
use shared::git::Git;
#[allow(dead_code)]
use shared::trigger::trigger;
use shared::types::TriggerTuple;
use std::env;
use std::error::Error;
use std::process::exit;

/// Launch attached subprocess
fn main() {
    handler().unwrap_or_else(|a| exit(1))
}
fn handler() -> Result<(), Box<dyn Error>> {
    // Retrieve action
    let args = env::args().collect::<Vec<String>>();
    let action: String = args[1].to_owned();

    // Retrieve branch
    let git = Git::new();
    let branch = git.branch().unwrap();

    let env = TriggerTuple {
        action: action,
        branch: branch,
    };
    trigger(&env)?;
    Ok(())
}
