#[cfg(test)]
mod cli {
    // Error Handling
    use miette::{IntoDiagnostic, Result};

    use assert_cmd::prelude::*; // Add methods on commands
    use predicates::prelude::*; // Used for writing assertions
    use std::process::Command; // Run commnds

    #[test]
    /// Run pipeline but no config found
    fn run_pipeline_no_config() -> Result<()> {
        let mut cmd = Command::cargo_bin("pipelight").into_diagnostic()?;
        cmd.arg("run")
            .arg("test_empty")
            .arg("--config")
            .arg("test.pipelight.ts");
        cmd.assert().failure();
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
