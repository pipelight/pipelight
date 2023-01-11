use crate::types::{Config, Pipeline, Trigger};
use log::{error, warn};
use std::error::Error;

trait ByName<T> {
    /// Return an instance of the struct.
    fn name(name: &str) -> Result<T, Box<dyn Error>>;
}
trait Computed<T> {
    /// Return every instances of the struct.
    fn get() -> Result<Vec<T>, Box<dyn Error>>;
}

impl Computed<Pipeline> for Pipeline {
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
}
impl Computed<Trigger> for Trigger {
    fn get() -> Result<Vec<Trigger>, Box<dyn Error>> {
        let pipelines = Pipeline::get()?;
        let mut triggers = pipelines
            .iter()
            .map(|p| p.triggers.clone().unwrap_or_default())
            .collect::<Vec<Vec<Trigger>>>()
            .into_iter()
            .flatten()
            .collect::<Vec<Trigger>>();
        triggers = triggers.sort();
        triggers = triggers.dedup();
        Ok(triggers)
    }
}
impl ByName<Pipeline> for Pipeline {
    fn name(name: &str) -> Result<Pipeline, Box<dyn Error>> {
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
