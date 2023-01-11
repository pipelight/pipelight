// Actions: Functions called by cli
use log::{debug, error, info, trace, warn};
use std::error::Error;
use utils::git::Hook;

pub fn init() -> Result<(), Box<dyn Error>> {
    Hook::ensure()?;
    Ok(())
}
