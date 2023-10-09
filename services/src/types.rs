// Struct
use actions::types::Action;
use cli::types::Cli;
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
    pub action: Action,
}

impl Service {
    pub fn new(action: Action, args: Option<Cli>) -> Result<Self> {
        let mut service = Service { action, args };
        service.convert()?;
        Ok(service)
    }
}
