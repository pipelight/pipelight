// Test
mod test;
// Error handling
use miette::{IntoDiagnostic, Result};

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
    /**
    Build and launch the custom watcher
    */
    #[tokio::main]
    pub async fn start() -> Result<()> {
        // Kill already running watcher
        Watcher::kill_homologous()?;
        let watchexec = build().await?;
        watchexec.main().await.into_diagnostic()??;
        Ok(())
    }
}
