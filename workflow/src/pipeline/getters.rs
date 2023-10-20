use crate::error::IsError;
use crate::traits::Getters;
use crate::types::{Config, Pipeline};
use exec::Process;
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
                let hint = "You may have made a typo";
                Err(IsError::new(&message, &hint)?.into())
            }
        }
    }
}

impl Pipeline {
    pub fn get_procs(&self) -> Result<Vec<Process>> {
        let mut procs: Vec<Process> = vec![];
        for step in self.steps.clone() {
            procs.extend(step.get_procs()?);
        }
        if let Some(fallback) = self.fallback.clone() {
            procs.extend(fallback.get_procs()?);
        }
        Ok(procs)
    }
}
