// Struct
use pipelight_error::{LibError, PipelightError, WrapError};
use pipelight_git::Flag;
// Global vars
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
// Teleport
use pipelight_teleport::Portal;
// Logs
use workflow::{Config, Trigger};
// Cli
use crate::types::Cli;
use crate::types::{Commands, DetachableCommands, PostCommands};
// Error Handling
use env_logger::Builder;
use log::{info, trace};
use miette::Result;

// Global vars
use crate::globals::CLI;
use pipelight_utils::globals::LOGGER;
use workflow::globals::CONFIG;

pub static PORTAL: Lazy<Arc<Mutex<Portal>>> = Lazy::new(|| Arc::new(Mutex::new(Portal::default())));

// Hydrate logs
pub fn early_hydrate_logger() -> Result<()> {
    let args = CLI.lock().unwrap().clone();
    // Set verbosity level
    let verbosity = args.verbose.log_level_filter();
    LOGGER.lock().unwrap().set_level(&verbosity)?;
    // Set internal verbosity level
    let verbosity = args.internal_verbose.log_level_filter();
    LOGGER.lock().unwrap().set_internal_level(&verbosity)?;
    std::env::set_var("PIPELIGHT_LOG", verbosity.to_string().to_lowercase());
    Builder::from_env("PIPELIGHT_LOG").init();

    Ok(())
}

// Hydrate trigger
pub fn hydrate_trigger() -> Result<()> {
    let args = CLI.lock().unwrap().clone();
    let mut flag = None;
    if let Commands::PostCommands(PostCommands::DetachableCommands(detachable_commands)) =
        args.commands
    {
        match detachable_commands {
            DetachableCommands::Trigger(trigger) => {
                flag = trigger.flag;
            }
            DetachableCommands::Run(pipeline) => {
                flag = pipeline.trigger.flag;
            }
            _ => {}
        }
    }
    if let Some(flag) = flag {
        Trigger::set(Some(Flag::from(&flag)))?;
    } else {
        Trigger::set(None)?;
    }
    Ok(())
}

// Hydrate portal
pub fn hydrate_portal() -> Result<()> {
    let args = CLI.lock().unwrap().clone();

    let seed = if args.config.is_some() {
        args.config.unwrap()
    } else {
        "pipelight".to_owned()
    };
    let mut portal = Portal::new()?;
    portal.seed(&seed);

    let res = portal.search();
    match res {
        Ok(portal) => {
            info!(
                "Load config file -> {}",
                portal.target.file_path.clone().unwrap()
            );
            *PORTAL.lock().unwrap() = portal;
            return Ok(());
        }
        Err(e) => {
            let message = "Could not find a configuration file".to_owned();
            let help = "Create a default configuration file: \"pipelight init --help\"".to_owned();
            return Err(PipelightError::WrapError(WrapError {
                message,
                help,
                origin: e.into(),
            })
            .into());
        }
    };
}

// Hydrate config
pub fn hydrate_config() -> Result<()> {
    let portal = PORTAL.lock().unwrap().clone();
    let args = CLI.lock().unwrap().clone();

    let casted_config = cast::Config::load(&portal.target.file_path.unwrap(), args.raw.clone())?;
    let config = Config::from(&casted_config);
    *CONFIG.lock().unwrap() = config.clone();

    Ok(())
}

/**
Read the command line and the config file
then hydrate every globals.
*/
pub fn set_globals() -> Result<()> {
    let cond = *CONFIG.lock().unwrap() == Config::default()
        && *PORTAL.lock().unwrap() == Portal::default();
    if cond {
        // hydrate the CLI global var
        Cli::hydrate()?;
        hydrate_trigger()?;
        // early_hydrate_logger()?;
        // hydrate the PORTAL global var
        hydrate_portal()?;
        // hydrate the CONFIG global var
        (*PORTAL.lock().unwrap()).teleport()?;
        hydrate_config()?;
    }
    Ok(())
}

/**
Only read the command line and ignore the config file
then hydrate globals that can be hydrated.
*/
pub fn set_early_globals() -> Result<()> {
    let cond = *CONFIG.lock().unwrap() == Config::default()
        && *PORTAL.lock().unwrap() == Portal::default();
    if cond {
        // hydrate the CLI global var
        Cli::hydrate()?;
        early_hydrate_logger()?;
    }
    Ok(())
}
