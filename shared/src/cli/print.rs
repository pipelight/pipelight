use chrono::{DateTime, Local};
use log::info;
use pipeline::types::{Config, Logs, Pipeline, Status};
use std::error::Error;

/// Pretty print logs from json log file
pub fn pretty() -> Result<(), Box<dyn Error>> {
    let pipelines = Logs::get()?;
    for mut pipeline in pipelines {
        if pipeline.is_aborted() {
            pipeline.status = Some(Status::Aborted);
            pipeline.event.as_mut().unwrap().pid = None;
            pipeline.log();
        }
        println!("{}", pipeline);
    }
    Ok(())
}

/// Print json log file
pub fn json() -> Result<(), Box<dyn Error>> {
    let pipelines = Logs::get()?;
    for pipeline in pipelines {
        let pipeline_json = serde_json::to_string::<Pipeline>(&pipeline)?;
        println!("{}", pipeline_json);
    }
    Ok(())
}

/// Print a flatten list of pipelines present in config file
pub fn list() -> Result<(), Box<dyn Error>> {
    // Retrieve pipelines defined in config and run logs.
    let config = Config::new();
    info!(target: "nude",
        "{:<10} {:<25} {:<40}\n",
        "status", "date" ,"name"
    );
    for pipeline in &config.pipelines.unwrap() {
        let pipe_logs = Logs::get_by_name(&pipeline.name)?;
        let pipe_last_log = pipe_logs.iter().next().unwrap().clone();
        let date;
        if pipe_last_log.status.is_some() {
            let str_date = pipe_last_log.event.unwrap().date;
            date = str_date
                .parse::<DateTime<Local>>()
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string();
        } else {
            date = "".to_owned();
        }
        println!(
            "{:<10} {:<25} {:<40}",
            String::from(&pipe_last_log.status.unwrap()),
            date,
            pipeline.name
        );
    }
    Ok(())
}
