// Error Handling
use miette::{MietteDiagnostic, Result};

/**
A json report type with hint, colors and code span for better pipeline debugging
*/
#[derive(Debug, Clone)]
pub struct IsError;
impl IsError {
    pub fn new(message: &str, help: &str) -> Result<MietteDiagnostic> {
        let diag = MietteDiagnostic::new(message)
            .with_code("worklow::is")
            .with_help(help);
        Ok(diag)
    }
}
