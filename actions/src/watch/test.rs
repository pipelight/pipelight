#[cfg(test)]
mod watcher {
    // Globals
    use crate::watch::build;
    use std::sync::Arc;
    // Watchexec
    use utils::files::Ignore;
    use utils::teleport::Portal;
    use watchexec::{
        action::{Action, Outcome},
        config::{InitConfig, RuntimeConfig},
        handler::{Handler as _, PrintDebug},
        Watchexec,
    };
    // Error handling
    use miette::{Diagnostic, IntoDiagnostic, Result};
    use thiserror::Error;

    #[tokio::test]
    async fn builder() -> Result<()> {
        // Teleport
        Portal::new()?.seed("test.pipelight").search()?.teleport()?;
        let res = build();
        assert!(res.is_ok());
        Ok(())
    }
}
