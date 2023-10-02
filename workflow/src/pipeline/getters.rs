use crate::traits::Getters;
use crate::types::{Config, Pipeline};

// Error Handling
use log::warn;
use miette::{Error, Result};

impl Getters<Pipeline> for Pipeline {
    /**
    Return pipelines from config file.
    */
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
    /**
    Check if pipeline with name exists, and return it.
    */
    fn get_by_name(name: &str) -> Result<Pipeline> {
        let pipelines = Pipeline::get()?;
        let optional = pipelines.iter().find(|p| p.name == name);
        match optional {
            Some(res) => Ok(res.to_owned()),
            None => {
                let message = format!("Couldn't find pipeline: {:?}", name);
                warn!("{}", message);
                Err(Error::msg(message))
            }
        }
    }
}
