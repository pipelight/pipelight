// Global vars
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
// Struct
use crate::types::{Config, Pipeline, Trigger};

/**
Here we use global variables
to avoid reading the same file multiple times.
*/

pub static CONFIG: Lazy<Arc<Mutex<Config>>> = Lazy::new(|| Arc::new(Mutex::new(Config::default())));
pub static TRIGGER_ENV: Lazy<Arc<Mutex<Trigger>>> =
    Lazy::new(|| Arc::new(Mutex::new(Trigger::default())));
pub static LOGS: Lazy<Arc<Mutex<Option<Vec<Pipeline>>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));
