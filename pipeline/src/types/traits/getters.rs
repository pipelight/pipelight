use crate::types::{Config, Logs, Pipeline, Trigger};
use log::warn;
use std::error::Error;
use std::fs;
use std::path::Path;
use utils::logger::logger;

pub trait Getters<T> {
    /// Return every instances of the struct.
    fn get() -> Result<Vec<T>, Box<dyn Error>>;
    /// Return an instance of the struct.
    fn get_by_name(name: &str) -> Result<T, Box<dyn Error>>;
}

impl Getters<Pipeline> for Logs {
    fn get() -> Result<Vec<Pipeline>, Box<dyn Error>> {
        let dir = &logger.lock().unwrap().directory;
        // Safe exit if no log folder
        if !Path::new(dir).exists() {
            let message = "No log can be displayed. Log folder doesn't exist";
            return Err(Box::from(message));
        }
        let paths = fs::read_dir(dir).unwrap();

        // println!("{:?}", paths);

        let mut pipelines = vec![];
        for path in paths {
            println!("{:?}", path);

            let dir_entry = path?;
            let json = utils::read_last_line(&dir_entry.path())?;
            let pipeline = serde_json::from_str::<Pipeline>(&json)?;
            pipelines.push(pipeline);
        }
        // pipelines = Logs::sanitize(pipelines)?;
        Ok(pipelines)
    }
    fn get_by_name(name: &str) -> Result<Pipeline, Box<dyn Error>> {
        let pipelines = Logs::get()?;
        let pipeline;
        let mut pipelines = pipelines
            .iter()
            .filter(|p| &p.name == name)
            .cloned()
            .collect::<Vec<Pipeline>>();
        if !pipelines.is_empty() {
            pipelines.sort_by_key(|e| e.clone().event.unwrap().date);
            pipeline = pipelines.pop().unwrap();
            Ok(pipeline)
        } else {
            let message = format!("Couldn't find a pipeline named {:?}, in logs", name);
            return Err(Box::from(message));
        }
    }
}
impl Logs {
    pub fn get_many_by_name(name: &str) -> Result<Vec<Pipeline>, Box<dyn Error>> {
        let pipelines = Logs::get()?;
        let mut pipelines = pipelines
            .iter()
            .filter(|p| &p.name == name)
            .cloned()
            .collect::<Vec<Pipeline>>();
        if !pipelines.is_empty() {
            pipelines.sort_by_key(|e| e.clone().event.unwrap().date);
            Ok(pipelines)
        } else {
            let message = format!("Couldn't find a pipeline named {:?}, in logs", name);
            return Err(Box::from(message));
        }
    }
}
impl Getters<Pipeline> for Pipeline {
    fn get() -> Result<Vec<Pipeline>, Box<dyn Error>> {
        let config = Config::new();
        let optional = config.pipelines;
        match optional {
            Some(p) => return Ok(p),
            None => {
                let message = "Couldn't retrieve pipelines";
                return Err(Box::from(message));
            }
        };
    }
    fn get_by_name(name: &str) -> Result<Pipeline, Box<dyn Error>> {
        let pipelines = Pipeline::get()?;
        let optional = pipelines.iter().filter(|p| p.name == name).next();
        match optional {
            Some(res) => return Ok(res.to_owned()),
            None => {
                let message = format!("Couldn't find pipeline {:?}", name);
                warn!("{}", message);
                return Err(Box::from(message));
            }
        };
    }
}

impl Getters<Trigger> for Trigger {
    fn get() -> Result<Vec<Trigger>, Box<dyn Error>> {
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
    fn get_by_name(name: &str) -> Result<Trigger, Box<dyn Error>> {
        let triggers = Trigger::get();
        let binding = triggers?;
        let trigger = binding.iter().next().unwrap();
        Ok(trigger.to_owned())
    }
}
