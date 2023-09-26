// Global vars
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

use std::env;

// Default shell to sh
pub static SHELL: Lazy<Arc<Mutex<String>>> = Lazy::new(|| Arc::new(Mutex::new("sh".to_owned())));
pub static OUTDIR: Lazy<Arc<Mutex<String>>> =
    Lazy::new(|| Arc::new(Mutex::new(".pipelight/_internals/out".to_owned())));

/// Return user session shell when possible
fn get_shell() -> String {
    let user_shell = env::var("SHELL");
    match user_shell {
        Ok(res) => {
            *SHELL.lock().unwrap() = res.to_owned();
            return res.to_owned();
        }
        Err(_) => (*SHELL.lock().unwrap()).to_owned(),
    }
}
