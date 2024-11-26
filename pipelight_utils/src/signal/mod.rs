use console::{Key, Term};
// Error Handling
use miette::{IntoDiagnostic, Result};
use std::io::Error;

use signal_hook;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use signal_hook::consts::signal::*;
use signal_hook::consts::TERM_SIGNALS;
use signal_hook::flag;
// A friend of the Signals iterator, but can be customized by what we want yielded about each
// signal.
use signal_hook::iterator::exfiltrator::WithOrigin;
use signal_hook::iterator::SignalsInfo;
use signal_hook::low_level;

use std::thread;

/**

Handle Ctrl-C and soft SIGTERMs
Restore the terminal to its defaults

Reasons:
The dialogue crate hides the command line cursor.
The cursor needs to be unhide
if process is stopped in the middle of a dialogue with Ctrl^C
*/
pub fn restore_term() -> Result<()> {
    thread::spawn(|| {
        let mut signals = SignalsInfo::<WithOrigin>::new(&*TERM_SIGNALS)
            .into_diagnostic()
            .unwrap();

        for info in &mut signals {
            match info.signal {
                term_sig => {
                    Term::stdout().show_cursor().into_diagnostic().unwrap();
                    // These are all the ones left
                    assert!(TERM_SIGNALS.contains(&term_sig));
                    break;
                }
            }
        }
    });
    Ok(())
}
