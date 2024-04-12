#[cfg(test)]
mod cli {
    // Error Handling
    use miette::{IntoDiagnostic, Result};

    use crate::services::types::{Action, Service};
    use crate::services::FgBg;
    use crate::types::Cli;
    // Clap - command line lib
    use clap::FromArgMatches;

    use assert_cmd::prelude::*; // Add methods on commands
    use std::process::Command; // Run commnds

    #[test]
    /// Run pipeline but no config found
    fn run_pipeline_no_config() -> Result<()> {
        let mut cmd = Command::cargo_bin("pipelight").into_diagnostic()?;
        cmd.arg("run")
            .arg("test_empty")
            .arg("--config")
            .arg("test.pipelight.unknown.ts");
        cmd.assert().failure();
        Ok(())
    }
    #[test]
    /// Run simple pipeline
    fn run_pipeline() -> Result<()> {
        let mut cmd = Command::cargo_bin("pipelight").into_diagnostic()?;
        cmd.arg("run")
            .arg("test")
            .arg("--config")
            .arg("test.pipelight.ts");
        cmd.assert().success();
        Ok(())
    }
    #[test]
    /// Display logs even when no config file present
    fn logs_pipeline() -> Result<()> {
        let mut cmd = Command::cargo_bin("pipelight").into_diagnostic()?;
        cmd.arg("logs");
        Ok(())
    }
    #[test]
    /// Generate auto completion even when no config file present
    fn make_completion() -> Result<()> {
        let mut cmd = Command::cargo_bin("pipelight").into_diagnostic()?;
        cmd.arg("completion");
        Ok(())
    }
}
