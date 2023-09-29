// Global vars
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
// Teleport
use utils::{logger::Logger, teleport::Portal};
// Logs
use workflow::{Config, Trigger};
// Cli
use clap::FromArgMatches;
use cli::types::Cli;
// Error Handling
use log::{info, trace};
use miette::Result;

pub static mut CLI: Lazy<Cli> = Lazy::new(Cli::new);
pub static LOGGER: Lazy<Arc<Mutex<Logger>>> = Lazy::new(|| Arc::new(Mutex::new(Logger::new())));
pub static mut CONFIG: Lazy<Config> = Lazy::new(Config::default);
pub static mut PORTAL: Lazy<Portal> = Lazy::new(Portal::default);
pub static mut TRIGGER_ENV: Lazy<Trigger> = Lazy::new(Trigger::default);

// Hydrate logs
pub fn early_hydrate_logger() -> Result<()> {
    let args;
    unsafe {
        args = (*CLI).clone();
    };
    // Set internal verbosity level
    let verbosity = args.verbose.log_level_filter();
    LOGGER.lock().unwrap().set_level(&verbosity)?;
    // Set verbosity level
    let verbosity = args.internal_verbose.log_level_filter();
    LOGGER.lock().unwrap().set_internal_level(&verbosity)?;
    Ok(())
}
// Hydrate logs
pub fn full_hydrate_logger() -> Result<()> {
    let mut portal;
    unsafe {
        portal = (*PORTAL).clone();
    };
    portal.teleport()?;
    LOGGER.lock().unwrap().to_file();
    portal.origin()?;
    Ok(())
}

// Hydrate cli
pub fn hydrate_cli() -> Result<()> {
    let cli = Cli::build()?;
    let matches = cli.get_matches();
    let args = Cli::from_arg_matches(&matches)
        .map_err(|err| err.exit())
        .unwrap();
    unsafe { *CLI = args.clone() };
    Ok(())
}

// Hydrate portal
pub fn hydrate_portal() -> Result<()> {
    let args;
    unsafe { args = (*CLI).clone() };
    let seed = if args.config.is_some() {
        args.config.unwrap()
    } else {
        "pipelight".to_owned()
    };
    let portal = Portal::new()?.seed(&seed).search()?;
    info!(
        "Found config file at: {}",
        portal.target.file_path.clone().unwrap()
    );
    unsafe {
        *PORTAL = portal.clone();
    };
    Ok(())
}

// Hydrate config
pub fn hydrate_config() -> Result<()> {
    let portal;
    let args;
    unsafe {
        portal = (*PORTAL).clone();
        args = (*CLI).clone();
    };
    let casted_config = cast::Config::load(&portal.target.file_path.unwrap(), args.raw.clone())?;
    let config = Config::from(&casted_config);
    unsafe { *CONFIG = config.clone() };
    Ok(())
}

// The main usage of teleport
// Set every main globals
pub fn set_globals() -> Result<()> {
    trace!("Set globals");
    let cond;
    unsafe { cond = *CONFIG == Config::default() && *PORTAL == Portal::default() };
    if cond {
        // hydrate the CLI global var
        hydrate_cli()?;
        early_hydrate_logger()?;
        // hydrate the PORTAL global var
        hydrate_portal()?;
        // hydrate the CONFIG global var
        unsafe {
            (*PORTAL).teleport()?;
        }
        full_hydrate_logger()?;
        hydrate_config()?;
    }
    Ok(())
}
pub fn set_early_globals() -> Result<()> {
    trace!("Set early globals");
    let cond;
    unsafe { cond = *CONFIG == Config::default() && *PORTAL == Portal::default() };
    if cond {
        // hydrate the CLI global var
        hydrate_cli()?;
        early_hydrate_logger()?;
    }
    Ok(())
}

// impl Config {
// pub fn get() -> Result<Self> {
// let config;
// unsafe { config = (*CONFIG).clone() };
// Ok(config)
// }
// }
