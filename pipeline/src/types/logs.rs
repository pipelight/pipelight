use super::{Logs, Pipeline};
use exec::types::{Status, StrOutput};
use log::{info, warn};
use std::error::Error;

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
}
