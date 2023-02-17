use exec::Exec;
use log::trace;
use pipeline::types::Logs;
use std::env;
use std::error::Error;

/// Launch attached subprocess
pub fn stop(pipeline_name: &String) -> Result<(), Box<dyn Error>> {
    let mut list = Logs::get_many_by_name(&pipeline_name)?;
    //Get latest element
    list.reverse();
    let pipeline = list.iter().next();

    if pipeline.is_some() {
        pipeline.unwrap().clone().stop();
        Ok(())
    } else {
        let message = format!("Pipeline {} isn't running", pipeline_name);
        Err(Box::from(message))
    }
}
