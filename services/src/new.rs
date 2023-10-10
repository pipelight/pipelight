// Struct
use cli::types::{Cli, Commands, DetachableCommands, Pipeline, PostCommands, Trigger, Watch};
use exec::Status;
use utils::git::Flag;
// use workflow::{Getters, Node, Pipeline};
// Traits
use crate::traits::Parser;
// Error Handling
use crate::types::Service;
use miette::{Error, Result};

impl Service {
    pub fn new(cmd: Commands, args: Option<Cli>) -> Result<Self> {
        let mut service = Service { cmd, args };
        service.convert()?;
        Ok(service)
    }
}
