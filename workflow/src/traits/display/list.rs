// Types
use crate::{
    pipeline::Filters, Config, Event, Getters, Logs, Node, Pipeline, Trigger, TriggerBranch,
};
use chrono::{DateTime, Local};
use pipelight_exec::{Statuable, Status};
use pipelight_utils::git::Flag;
use serde::{Deserialize, Serialize};

use owo_colors::OwoColorize;
use std::fmt;
use tabled::{
    settings::{location::ByColumnName, object::Columns, Disable, Style},
    Table, Tabled,
};
//Logger
use log::{log_enabled, Level, LevelFilter};
use pipelight_utils::globals::LOGGER;
// Error Handling
use miette::{IntoDiagnostic, Result};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Tabled)]
pub struct PipelineTable {
    pub name: String,
    #[tabled(display_with = "display_status")]
    pub status: Option<Status>,
    // Event
    #[tabled(rename = "branch/tag", display_with = "display_string")]
    pub git_ref: Option<String>,
    #[tabled(display_with = "display_action")]
    pub action: Option<Flag>,
    #[tabled(display_with = "display_date")]
    pub date: Option<String>,
    #[tabled(display_with = "display_string")]
    pub commit: Option<String>,
}
impl PipelineTable {
    fn from(e: &Pipeline) -> Result<PipelineTable> {
        let mut table = PipelineTable {
            name: e.name.clone(),
            status: e.status.clone(),
            // Flattened Event
            action: None,
            date: None,
            git_ref: None,
            commit: None,
        };
        // Try to retrieve info from last logs
        let log = Logs::get_by_name(&e.name);
        if let Ok(log) = log {
            if let Some(event) = log.event.clone() {
                table.date = Some(event.date);
                table.action = event.trigger.get_action()?;
                table.git_ref = event.trigger.get_ref()?;
                table.commit = event.trigger.get_commit()?;
            }
            table.status = log.status.clone();
            // table.last_trigger = log.triggers.clone();
        }
        Ok(table)
    }
}

impl PipelineTable {
    pub fn display(items: Vec<Self>, level: &LevelFilter) -> Result<()> {
        match level {
            LevelFilter::Error => {
                let mut res = Table::new(&items);
                res
                    // disable unwanted column
                    .with(Disable::column(ByColumnName::new("triggers")))
                    .with(Disable::column(ByColumnName::new("commit")))
                    .with(Disable::column(ByColumnName::new("date")))
                    .with(Disable::column(ByColumnName::new("action")))
                    .with(Disable::column(ByColumnName::new("branch/tag")))
                    .with(Disable::column(ByColumnName::new("status")));

                res.with(Style::rounded());
                println!("{}", res);
            }
            LevelFilter::Warn => {
                let mut res = Table::new(&items);
                res
                    // disable unwanted column
                    .with(Disable::column(ByColumnName::new("triggers")))
                    .with(Disable::column(ByColumnName::new("commit")))
                    .with(Disable::column(ByColumnName::new("date")))
                    .with(Disable::column(ByColumnName::new("action")))
                    .with(Disable::column(ByColumnName::new("branch/tag")));

                res.with(Style::rounded());
                println!("{}", res);
            }
            LevelFilter::Info => {
                let mut res = Table::new(&items);
                res
                    // disable unwanted column
                    .with(Disable::column(ByColumnName::new("triggers")))
                    .with(Disable::column(ByColumnName::new("commit")))
                    .with(Disable::column(ByColumnName::new("date")));

                res.with(Style::rounded());
                println!("{}", res);
            }
            LevelFilter::Debug => {
                let mut res = Table::new(&items);
                res
                    // disable unwanted column
                    .with(Disable::column(ByColumnName::new("triggers")))
                    .with(Disable::column(ByColumnName::new("commit")));

                res.with(Style::rounded());
                println!("{}", res);
            }
            LevelFilter::Trace => {
                let mut res = Table::new(&items);
                res.with(Style::rounded());
                res.with(Disable::column(ByColumnName::new("triggers")));
                println!("{}", res);
            }
            _ => {}
        };
        Ok(())
    }
}
pub fn display_status(e: &Option<Status>) -> String {
    if let Some(status) = e {
        return status.to_string();
    } else {
        return "-".white().to_string();
    }
}
pub fn display_date(e: &Option<String>) -> String {
    if let Some(date) = e {
        let date = date
            .parse::<DateTime<Local>>()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S");
        return date.to_string();
    } else {
        return "-".white().to_string();
    }
}
pub fn display_string(e: &Option<String>) -> String {
    if let Some(string) = e {
        return string.to_string();
    } else {
        return "-".white().to_string();
    }
}
pub fn display_action(e: &Option<Flag>) -> String {
    if let Some(action) = e {
        return action.to_string();
    } else {
        return "-".white().to_string();
    }
}

impl Pipeline {
    /**
    Pretty print pipelines as a tree
    */
    pub fn list_pretty(name: Option<String>) -> Result<()> {
        let mut pipelines = Pipeline::get()?;
        if let Some(name) = name {
            pipelines = Filters::filter_by_name(pipelines, &name)?;
        }
        for mut pipeline in pipelines {
            if pipeline.get_status() == Some(Status::Running) {
                pipeline.hydrate()?;
            }
            let node = Node::from(&pipeline.clone());
            print!("{}", node);
        }
        Ok(())
    }

    /**
    Pretty print pipelines as json
    */
    pub fn list_json(name: Option<String>) -> Result<()> {
        let mut pipelines = Pipeline::get()?;
        if let Some(name) = name {
            pipelines = Filters::filter_by_name(pipelines, &name)?;
        }
        for pipeline in pipelines {
            let pipeline_json =
                serde_json::to_string_pretty::<Pipeline>(&pipeline).into_diagnostic()?;
            print!("{}", pipeline_json);
        }
        Ok(())
    }

    /// Print pipeline from config file
    pub fn inspect(name: &str, json: bool) -> Result<()> {
        // Set logger level
        LOGGER.lock().unwrap().pipelines.level = LevelFilter::max();
        let pipeline = Pipeline::get_by_name(name)?;
        if json {
            let pipeline_json =
                serde_json::to_string_pretty::<Pipeline>(&pipeline).into_diagnostic()?;
            print!("{}", pipeline_json);
        } else {
            let node = Node::from(&pipeline);
            print!("{}", node);
        }
        Ok(())
    }

    /**
     * Print a flatten list of pipelines from the config file
     */
    pub fn list() -> Result<()> {
        let level = LOGGER.lock().unwrap().pipelines.level;
        let config = Config::get()?;

        // Retrieve pipelines defined in config files
        // and associated logs
        let mut table: Vec<PipelineTable> = vec![];
        for e in &config.pipelines.unwrap() {
            table.push(PipelineTable::from(e)?);
        }
        PipelineTable::display(table, &level)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default() -> Result<()> {
        let items = vec![PipelineTable {
            name: "test".to_owned(),
            status: Some(Status::Started),
            // Event
            date: None,
            action: None,
            git_ref: None,
            commit: None,
        }];
        let level = LevelFilter::Error;

        println!();
        PipelineTable::display(items, &level)?;
        Ok(())
    }
    #[test]
    fn max_log() -> Result<()> {
        let items = vec![PipelineTable {
            name: "test".to_owned(),
            status: Some(Status::Started),
            // Event
            date: None,
            action: None,
            git_ref: None,
            commit: None,
        }];
        let level = LevelFilter::max();

        println!();
        PipelineTable::display(items, &level)?;
        Ok(())
    }
}
