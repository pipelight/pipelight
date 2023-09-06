// Global vars
use crate::types::Trigger;
use once_cell::sync::Lazy;
// Teleport
use utils::{git::Hook, teleport::Portal};
// Logs
use cast;
// Cli
use crate::cli::interface::Cli;
use crate::types::Config;
use clap::FromArgMatches;
// Error Handling
use miette::{IntoDiagnostic, Result};

pub static mut CLI: Lazy<Cli> = Lazy::new(Cli::new);
pub static mut LOGS: Lazy<Cli> = Lazy::new(Cli::new);
pub static mut CONFIG: Lazy<Config> = Lazy::new(Config::default);
pub static mut PORTAL: Lazy<Portal> = Lazy::new(Portal::default);
pub static mut TRIGGER_ENV: Lazy<Trigger> = Lazy::new(Trigger::default);

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

// Hydrate logs
pub fn hydrate_logs() {}

// Hydrate config
pub fn hydrate_config(file_path: &str) -> Result<()> {
    let args;
    unsafe {
        args = *CLI;
    };
    let casted_config = cast::Config::load(file_path, args.raw.clone())?;
    let config = Config::from(casted_config);
    unsafe { *CONFIG = config.clone() };
    Ok(())
}

// The main usage of teleport
// Set every main globals
pub fn set_globals() -> Result<()> {
    unsafe {
        if *CONFIG == Config::default() && *PORTAL == Portal::default() {
            hydrate_cli()?;

            *PORTAL.hydrate_config()?;
            config = Config::from(&json);
            config.dedup_pipelines();

            *CONFIG = config;
            *PORTAL = portal;
        }
        let ptr = (*CONFIG).to_owned();
        let tel = (*PORTAL).to_owned();

        Ok((ptr, tel))
    }
}
