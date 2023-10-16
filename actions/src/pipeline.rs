// Types
use chrono::{DateTime, Local};
use exec::{Statuable, Status};
use workflow::{pipeline::Filters, Config, Getters, Logs, Node, Pipeline, Trigger};
//Logger
use log::{error, info, warn, LevelFilter};
use utils::globals::LOGGER;
// Error Handling
use miette::{IntoDiagnostic, Result};

/**
Pretty print pipelines as a tree
*/
pub fn pretty(name: Option<String>) -> Result<()> {
    let mut pipelines = Pipeline::get()?;
    if let Some(name) = name {
        pipelines = Filters::filter_by_name(pipelines, &name)?;
    }
    for mut pipeline in pipelines {
        if pipeline.get_status() == Some(Status::Running) {
            pipeline.hydrate()?;
        }
        let node = Node::from(&pipeline.clone());
        println!("{}", node);
    }
    Ok(())
}

/**
Pretty print pipelines as json
*/
pub fn json(name: Option<String>) -> Result<()> {
    let mut pipelines = Pipeline::get()?;
    if let Some(name) = name {
        pipelines = Filters::filter_by_name(pipelines, &name)?;
    }
    for pipeline in pipelines {
        let pipeline_json =
            serde_json::to_string_pretty::<Pipeline>(&pipeline).into_diagnostic()?;
        println!("{}", pipeline_json);
    }
    Ok(())
}

/// Print pipeline from config file
pub fn inspect(name: &str, json: bool) -> Result<()> {
    // Set logger level
    LOGGER.lock().unwrap().pipelines.level = LevelFilter::max();
    let pipeline = Pipeline::get_by_name(&name)?;

    if json {
        let pipeline_json =
            serde_json::to_string_pretty::<Pipeline>(&pipeline).into_diagnostic()?;
        println!("{}", pipeline_json);
    } else {
        let node = Node::from(&pipeline);
        println!("{}", node);
    }
    Ok(())
}

/**
Print a flatten list of pipelines from the config file
*/
pub fn default() -> Result<()> {
    let level = LOGGER.lock().unwrap().pipelines.level;
    let config = Config::get()?;
    let logs = Logs::get()?;

    // Print headers
    match level {
        LevelFilter::Warn => {
            warn!(target: "pipelines_nude",
                "{:<15} {:<25} {:<40}\n",
                "status","date" ,"name"
            );
        }
        _ => {
            info!(target: "pipelines_nude",
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
        let last_log = Logs::get_by_name(&pipeline.name);
        if let Ok(last_log) = last_log {
            status = String::from(&last_log.status.clone().unwrap());

            let event = last_log.event.clone().unwrap();
            match event.trigger.clone() {
                Trigger::TriggerBranch(trigger_branch) => {
                    if let Some(binding_branch) = trigger_branch.branch {
                        branch = String::from(&binding_branch);
                    }
                }
                _ => {}
            };
            action = String::from(&event.trigger.get_action()?.unwrap());
            let str_date = &event.date;
            date = str_date
                .parse::<DateTime<Local>>()
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string();
        }
        match level {
            LevelFilter::Warn => {
                warn!(target: "pipelines_nude",
                    "{:<15} {:<25} {:<40}\n",
                    status, date, pipeline.name);
            }
            LevelFilter::Error => {
                error!(target: "pipelines_nude",
                    "{:<40}\n",
                     pipeline.name);
            }
            _ => {
                info!(target: "pipelines_nude",
                    "{:<15} {:<15} {:<15} {:<25} {:<40}\n",
                    status, action, branch, date, pipeline.name
                );
            }
        }
    }
    Ok(())
}
