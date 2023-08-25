mod logs;

use crate::types::{Config, Pipeline, Trigger};

// Logger
use log::warn;

// Error Handling
use miette::{Error, Result};
// use std::error::Error;

// Import global config
use super::default::{CONFIG, TELEPORT};

// External imports
use utils::git::Hook;
use utils::teleport::Teleport;

pub trait Getters<T> {
    /// Return every instances of the struct.
    fn get() -> Result<Vec<T>>;
    /// Return an instance of the struct.
    fn get_by_name(name: &str) -> Result<T>;
}

impl Config {
    pub fn get() -> Result<Self> {
        let (config, portal) = Config::get_with_teleport()?;
        Ok(config)
    }
    pub fn get_with_teleport() -> Result<(Self, Teleport)> {
        unsafe {
            if *CONFIG != Config::default() && *TELEPORT != Teleport::default() {
                let config = (*CONFIG).to_owned();
                let teleport = (*TELEPORT).to_owned();
                Ok((config, teleport))
            } else {
                Err(Error::msg("Config file not initialized"))
            }
        }
    }
    pub fn new(file: Option<String>, args: Option<Vec<String>>) -> Result<Self> {
        let (config, portal) = Config::new_with_teleport(file, args)?;
        Hook::enable()?;
        // Launch watcher
        Ok(config)
    }
    pub fn new_with_teleport(
        file: Option<String>,
        args: Option<Vec<String>>,
    ) -> Result<(Self, Teleport)> {
        unsafe {
            if *CONFIG == Config::default() && *TELEPORT == Teleport::default() {
                let mut config: Config;
                let (json, portal) = cast::Config::get_with_teleport(file, args)?;

                config = Config::from(&json);
                config.dedup_pipelines();
                *CONFIG = config;
                *TELEPORT = portal;
            }
            let ptr = (*CONFIG).to_owned();
            let tel = (*TELEPORT).to_owned();

            Ok((ptr, tel))
        }
    }
}

impl Getters<Pipeline> for Pipeline {
    fn get() -> Result<Vec<Pipeline>> {
        let config = Config::get()?;
        let optional = config.pipelines;
        match optional {
            Some(p) => Ok(p),
            None => {
                let message = "Couldn't retrieve pipelines";
                Err(Error::msg(message))
            }
        }
    }
    fn get_by_name(name: &str) -> Result<Pipeline> {
        let pipelines = Pipeline::get()?;
        let optional = pipelines.iter().find(|p| p.name == name);
        match optional {
            Some(res) => Ok(res.to_owned()),
            None => {
                let message = format!("Couldn't find pipeline {:?}", name);
                warn!("{}", message);
                Err(Error::msg(message))
            }
        }
    }
}

impl Getters<Trigger> for Trigger {
    fn get() -> Result<Vec<Trigger>> {
        let pipelines = Pipeline::get()?;
        let mut triggers = pipelines
            .iter()
            .map(|p| p.triggers.clone().unwrap_or_default())
            .collect::<Vec<Vec<Trigger>>>()
            .into_iter()
            .flatten()
            .collect::<Vec<Trigger>>();
        triggers.sort();
        triggers.dedup();
        Ok(triggers)
    }
    fn get_by_name(name: &str) -> Result<Trigger> {
        let triggers = Trigger::get();
        let binding = triggers?;
        let trigger = binding.first().unwrap();
        Ok(trigger.to_owned())
    }
}
