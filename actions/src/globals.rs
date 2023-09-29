// Global vars
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
//Struct
use cli::types::Cli;

pub static CLI: Lazy<Arc<Mutex<Cli>>> = Lazy::new(|| Arc::new(Mutex::new(Cli::new())));
