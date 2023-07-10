#![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(dead_code)]
#![allow(unused_must_use)]

use cli::case::Client;

use log::error;
use std::process::exit;

// Error Handling
use miette::Result;

fn main() {
    handler().unwrap_or_else(|e| {
        error!("{}", e);
        exit(1)
    })
}
fn handler() -> Result<()> {
    Client::launch()?;
    Ok(())
}
