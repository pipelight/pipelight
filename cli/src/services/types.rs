// Struct
use crate::services::traits::Parser;
use crate::types::{Cli, Commands};
// Error Handling
use miette::Result;

/**
If you want to create a detached or attached running process fork.
If you want to execute actions in the same process,
use Actions directly.
*/
#[derive(Debug, Clone)]
pub struct Service {
    pub args: Option<Cli>,
    pub cmd: Commands,
}

impl Service {
    pub fn new(cmd: Commands, args: Option<Cli>) -> Result<Self> {
        let mut service = Service { cmd, args };
        service.convert()?;
        Ok(service)
    }
}
