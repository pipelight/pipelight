// Test
mod test;

// Error handling
use miette::{Diagnostic, IntoDiagnostic, Result};

mod build;

// mod builder;
// pub use builder::*;
pub use build::*;
mod is;

#[derive(Debug)]
pub struct Watcher;

impl Watcher {
    pub fn kill() -> Result<()> {
        Watcher::kill_homologous()?;
        Ok(())
    }
}
