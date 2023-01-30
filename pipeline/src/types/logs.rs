use super::{Logs, Pipeline};
use exec::types::{Status, StrOutput};
use log::{info, warn};
use std::error::Error;
use std::fs;
use std::path::Path;
use utils::logger::logger;

impl Logs {
    fn sanitize(pipelines: Vec<Pipeline>) -> Result<Vec<Pipeline>, Box<dyn Error>> {
        let message = "Sanitizing log files";
        info!("{}", message);
        for mut pipeline in pipelines.clone() {
            if pipeline.is_aborted() {
                pipeline.status = Some(Status::Aborted);
                pipeline.log();
            }
        }
        Ok(pipelines.to_owned())
    }
    /// Return pipelines from log files
    pub fn get() -> Result<Vec<Pipeline>, Box<dyn Error>> {
        let dir = &logger.load().directory;
        // Safe exit if no log folder
        if !Path::new(dir).exists() {
            let message = "No log can be displayed. Log folder is empty";
            return Err(Box::from(message));
        }
        let paths = fs::read_dir(dir).unwrap();
        let mut pipelines = vec![];
        for res in paths {
            let dir_entry = res?;
            let json = utils::read_last_line(&dir_entry.path())?;
            let pipeline = serde_json::from_str::<Pipeline>(&json)?;
            pipelines.push(pipeline);
        }
        // pipelines = Logs::sanitize(pipelines)?;
        Ok(pipelines)
    }
    pub fn get_by_name(name: &String) -> Result<Vec<Pipeline>, Box<dyn Error>> {
        let pipelines = Logs::get()?;
        let mut pipelines = pipelines
            .iter()
            .filter(|p| &p.name == name)
            .cloned()
            .collect::<Vec<Pipeline>>();
        if !pipelines.is_empty() {
            pipelines.sort_by_key(|e| e.clone().event.unwrap().date);
        } else {
            warn!("Couldn't find a pipeline named {:?}", name);
        }
        Ok(pipelines)
    }
}
