// Actions: Functions called by cli
use exec::Exec;
use log::{debug, error, info, trace, warn};
use pipeline::cast::Config;
use std::error::Error;
use utils::git::Hooks;

pub fn init() -> Result<(), Box<dyn Error>> {
    Hooks::ensure()?;
    Ok(())
}
