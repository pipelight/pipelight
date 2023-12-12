// Struct
use crate::services::types::{Action, Service};
// Actions
use crate::actions;
// Error Handling
use log::trace;
use miette::Result;

pub trait Exec {
    /**
    Execute the service action
    */
    fn exec(&self) -> Result<()>;
}
impl Exec for Service {
    fn exec(&self) -> Result<()> {
        match self.cmd {
            Action::RunStrict => actions::run::strict::launch()?,
            Action::RunLoose => actions::run::loose::launch()?,
            Action::Trigger => actions::trigger::launch()?,
            Action::Watch => actions::watch::Watcher::start()?,
        };
        Ok(())
    }
}
