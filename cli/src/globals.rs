// Global vars
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
// Teleport
use utils::{logger::Logger, teleport::Portal};
// Logs
use cast;
use workflow::{Config, Trigger};
// Cli
use crate::types::Cli;
use clap::FromArgMatches;
// Error Handling
use log::{info, trace};
use miette::Result;

pub static mut CLI: Lazy<Cli> = Lazy::new(Cli::new);
