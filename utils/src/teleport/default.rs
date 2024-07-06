// Structs
use crate::teleport::types::{Gate, Portal};
// Environment
use std::env;
// Error Handling
use miette::Result;

impl Portal {
    /**
    Preffered way to instanciate a portal.
    Hydrate a default portal with current env.
    */
    pub fn new() -> Result<Self> {
        Ok(Portal {
            target: Gate::default(),
            origin: Gate::default().directory(env::current_dir().unwrap().display().to_string())?,
            current: Gate::default()
                .directory(env::current_dir().unwrap().display().to_string())?,
            seed: None,
        })
    }
}
