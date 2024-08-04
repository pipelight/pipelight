pub mod loose;
pub mod strict;

// Global vars
use once_cell::sync::Lazy;
use std::process::ExitCode;
use std::sync::{Arc, Mutex};

pub static EXIT_CODE: Lazy<Arc<Mutex<ExitCode>>> =
    Lazy::new(|| Arc::new(Mutex::new(ExitCode::default())));
