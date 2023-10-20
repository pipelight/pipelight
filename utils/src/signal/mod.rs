use console::{Key, Term};
use signal_hook;
// Error Handling
use miette::{IntoDiagnostic, Result};

/**
Show the cursor on the command line.

The dialogue crate hides the command line cursor.
The cursor needs to be unhide
if process is stopped in the middle of a dialogue with Ctrl^C
*/
pub fn restore_term() -> Result<()> {
    Term::stdout().show_cursor().into_diagnostic()?;
    Ok(())
}
