use crate::types::{DetachableCommands, PostCommands};
use actions::{inspect, prompt, run, stop, trigger, watch};
// Test
// mod test;
// Error Handling
use miette::Result;

impl DetachableCommands {
    pub fn start(&self) -> Result<()> {
        match self {
            DetachableCommands::Run(e) => {
                if let Some(name) = e.name {
                    run::launch(&name)?;
                } else {
                    // Select prompt
                    let name = prompt::pipeline()?;
                    run::launch(&name)?;
                }
            }
            DetachableCommands::Trigger(e) => {
                if let Some(e) = e.flag {
                    // Set global args flag ??
                }
                trigger::launch()?;
            }
            DetachableCommands::Watch(e) => {
                watch::Watcher::start()?;
            }
        }
        Ok(())
    }
}

impl PostCommands {
    pub fn start(&self) -> Result<()> {
        match self {
            PostCommands::Stop(e) => {
                if let Some(name) = e.name {
                    stop::launch(&name)?;
                } else {
                    // Select prompt
                    let name = prompt::pipeline()?;
                    stop::launch(&name)?;
                }
            }
            PostCommands::Inspect(e) => {
                if let Some(name) = e.name {
                    inspect::launch(&name)?;
                } else {
                    // Select prompt
                    let name = prompt::pipeline()?;
                    inspect::launch(&name)?;
                }
            }
        };
        Ok(())
    }
}
