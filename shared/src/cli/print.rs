use chrono::{DateTime, Local};
use log::{debug, error, info, warn, LevelFilter};
use pipeline::types::{Config, Logs, Pipeline};
use std::error::Error;
use utils::logger::logger;

/// Pretty print logs from json log file
pub fn pretty(pipelines: &Vec<Pipeline>) -> Result<(), Box<dyn Error>> {
    for pipeline in pipelines {
        println!("{}", pipeline);
    }
    Ok(())
}

/// Print pipeline from json log file
pub fn json(pipelines: &Vec<Pipeline>) -> Result<(), Box<dyn Error>> {
    for pipeline in pipelines {
        let pipeline_json = serde_json::to_string_pretty::<Pipeline>(&pipeline)?;
        println!("{}", pipeline_json);
    }
    Ok(())
}
/// Print pipeline from config file
pub fn inspect(pipeline: &Pipeline, json: bool) -> Result<(), Box<dyn Error>> {
    if json {
        let pipeline_json = serde_json::to_string_pretty::<Pipeline>(pipeline)?;
        println!("{}", pipeline_json);
    } else {
        println!("{}", pipeline);
    }
    Ok(())
}

/// Print a flatten list of pipelines present in config file
pub fn list() -> Result<(), Box<dyn Error>> {
    let level = logger.lock().unwrap().level;
    let config = Config::new();
    // Print headers
    match level {
        LevelFilter::Warn => {
            warn!(target: "nude",
                "{:<15} {:<25} {:<40}\n",
                "status","date" ,"name"
            );
        }
        LevelFilter::Error => {
            error!(target: "nude",
                "{:<40}\n",
                "name"
            );
        }
        _ => {
            info!(target: "nude",
                "{:<15} {:<15} {:<15} {:<25} {:<40}\n",
                "status", "action", "branch","date" ,"name"
            );
        }
    }
    // Retrieve pipelines defined in config and run logs.
    for pipeline in &config.pipelines.unwrap() {
        let mut date = "".to_owned();
        let mut status = "".to_owned();
        let mut action = "".to_owned();
        let mut branch = "".to_owned();
        // Retrieve logs data if any
        if Logs::get().is_ok() {
            let pipe_logs = Logs::get_by_name(&pipeline.name)?;
            let last_log = pipe_logs.iter().next();
            if last_log.is_some() {
                let last_log = last_log.unwrap();
                status = String::from(&last_log.status.clone().unwrap());
                branch = String::from(&last_log.event.clone().unwrap().trigger.branch.unwrap());
                action = String::from(&last_log.event.clone().unwrap().trigger.action.unwrap());
                let str_date = last_log.event.clone().unwrap().date;
                date = str_date
                    .parse::<DateTime<Local>>()
                    .unwrap()
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string();
            }
        }
        match level {
            LevelFilter::Warn => {
                warn!(target: "nude",
                    "{:<15} {:<25} {:<40}\n",
                    status, date, pipeline.name);
            }
            LevelFilter::Error => {
                error!(target: "nude",
                    "{:<40}\n",
                     pipeline.name);
            }
            _ => {
                info!(target: "nude",
                    "{:<15} {:<15} {:<15} {:<25} {:<40}\n",
                    status, action, branch, date, pipeline.name
                );
            }
        }
    }
    Ok(())
}
