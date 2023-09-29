// Global vars
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

// Teleport
use utils::logger::Logger;

use cli::types::Cli;

pub static LOGGER: Lazy<Arc<Mutex<Logger>>> = Lazy::new(|| Arc::new(Mutex::new(Logger::new())));
pub static CLI: Lazy<Arc<Mutex<Cli>>> = Lazy::new(|| Arc::new(Mutex::new(Cli::new())));
