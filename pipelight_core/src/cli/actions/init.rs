use utils::git::Hook;
// Error Handling
use miette::{IntoDiagnostic, Result};

// Enable hooks
pub fn enable_git_hooks() -> Result<()> {
    Hook::enable()?;
    Ok(())
}

// Create file
// Option API

// Helper API
