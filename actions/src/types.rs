// Struct
use cli::types::Cli;
// Error Handling
use miette::Result;

#[derive(Debug, Clone)]
pub enum Action {
    // Parameter: pipeline name
    Run(Option<String>),
    // Parameter: flag name
    Trigger(Option<String>),
    Watch,
}

#[derive(Debug, Clone)]
pub struct Run;

impl Action {
    fn start(&self) -> Result<()> {
        println!("{:#?}", self);
        Ok(())
    }
}
