// Global vars
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
// Struct
use crate::types::{Config, Trigger};

pub static CONFIG: Lazy<Arc<Mutex<Config>>> = Lazy::new(|| Arc::new(Mutex::new(Config::default())));
pub static TRIGGER_ENV: Lazy<Arc<Mutex<Trigger>>> =
    Lazy::new(|| Arc::new(Mutex::new(Trigger::default())));
