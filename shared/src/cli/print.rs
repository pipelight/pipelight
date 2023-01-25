use chrono::{DateTime, Local};
use log::{info, warn};
use pipeline::types::{Config, Logs, Pipeline, Status};
use std::error::Error;

/// Pretty print logs from json log file
pub fn pretty(pipelines: &Vec<Pipeline>) -> Result<(), Box<dyn Error>> {
    for pipeline in pipelines {
        println!("{}", pipeline);
    }
    Ok(())
}

/// Print json log file
pub fn json(pipelines: &Vec<Pipeline>) -> Result<(), Box<dyn Error>> {
    for pipeline in pipelines {
        let pipeline_json = serde_json::to_string_pretty::<Pipeline>(&pipeline)?;
        println!("{}", pipeline_json);
    }
    Ok(())
}

/// Print a flatten list of pipelines present in config file
pub fn list() -> Result<(), Box<dyn Error>> {
    // Retrieve pipelines defined in config and run logs.
    let config = Config::new();
    warn!(target: "nude",
        "{:<15} {:<25} {:<40}\n",
        "last_status", "last_run_date" ,"name"
    );
    for pipeline in &config.pipelines.unwrap() {
        let mut date = "".to_owned();
        let mut status = "".to_owned();
        // Retrieve logs data if any
        if Logs::get().is_ok() {
            let pipe_logs = Logs::get_by_name(&pipeline.name)?;
            let pipe_last_log = pipe_logs.iter().next().unwrap().clone();
            if pipe_last_log.status.is_some() {
                status = String::from(&pipe_last_log.status.unwrap());
                let str_date = pipe_last_log.event.unwrap().date;
                date = str_date
                    .parse::<DateTime<Local>>()
                    .unwrap()
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string();
            }
        }
        println!("{:<15} {:<25} {:<40}", status, date, pipeline.name);
    }
    Ok(())
}
