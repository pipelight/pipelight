use chrono::{DateTime, Local};
use log::{error, info, warn, LevelFilter};
use pipeline::{Config, Getters, Logs, Node, Pipeline};

// Error Handling
use miette::{IntoDiagnostic, Result};
// use std::error::Error;
//
//Logger
use utils::logger::logger;

/// Pretty print logs from json log file
pub fn pretty(pipelines: &Vec<Pipeline>) -> Result<()> {
    for pipeline in pipelines {
        let node = Node::from(pipeline);
        println!("{}", node);
    }
    Ok(())
}

/// Print pipeline from json log file
pub fn json(pipelines: &Vec<Pipeline>) -> Result<()> {
    for pipeline in pipelines {
        let pipeline_json =
            serde_json::to_string_pretty::<Pipeline>(&pipeline).into_diagnostic()?;
        println!("{}", pipeline_json);
    }
    Ok(())
}
/// Print pipeline from config file
pub fn inspect(pipeline: &Pipeline, json: bool) -> Result<()> {
    if json {
        let pipeline_json = serde_json::to_string_pretty::<Pipeline>(pipeline).into_diagnostic()?;
        println!("{}", pipeline_json);
    } else {
        let node = Node::from(pipeline);
        println!("{}", node);
    }
    Ok(())
}

/// Print a flatten list of pipelines present in config file
pub fn list() -> Result<()> {
    let level = logger.lock().unwrap().level;
    let config = Config::get()?;
    // Print headers
    match level {
        LevelFilter::Warn => {
            warn!(target: "nude",
                "{:<15} {:<25} {:<40}\n",
                "status","date" ,"name"
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
            let last_log = Logs::get_by_name(&pipeline.name);
            if last_log.is_ok() {
                let last_log = last_log.unwrap();
                status = String::from(&last_log.status.clone().unwrap());
                let event = last_log.event.clone().unwrap();
                if event.trigger.branch().is_some() {
                    branch = String::from(&event.trigger.branch().unwrap());
                }
                action = String::from(&event.trigger.action().unwrap());
                let str_date = &event.date;
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
