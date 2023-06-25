// # Module description
//
// This module is to find and read config files (ex: pipelight.<file_extension>)
//
// Files are converted into intermediate rust structs with the serde crate.
// Those intermediate structs are practical to define a config file
// but harsh to use as is in a rust ecosystem.
//
// They are only used here to cast config files
// and are then converted into practical structs to be used outside the crate.

//import modules
mod config;
mod default;
mod error;
mod typescript;

use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Fallback {
    pub on_started: Option<Vec<StepOrParallel>>,
    pub on_failure: Option<Vec<StepOrParallel>>,
    pub on_success: Option<Vec<StepOrParallel>>,
    pub on_abortion: Option<Vec<StepOrParallel>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub pipelines: Option<Vec<Pipeline>>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Pipeline {
    pub name: String,
    pub triggers: Option<Vec<Trigger>>,
    pub steps: Vec<StepOrParallel>,
    #[serde(flatten)]
    pub fallback: Option<Fallback>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Step {
    pub name: String,
    pub commands: Vec<String>,
    pub mode: Option<String>,
    #[serde(flatten)]
    pub fallback: Option<Fallback>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Parallel {
    pub parallel: Vec<Step>,
    pub mode: Option<String>,
    #[serde(flatten)]
    pub fallback: Option<Fallback>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum StepOrParallel {
    Step(Step),
    Parallel(Parallel),
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Trigger {
    TriggerBranch(TriggerBranch),
    TriggerTag(TriggerTag),
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct TriggerBranch {
    pub branches: Option<Vec<String>>,
    pub actions: Option<Vec<String>>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct TriggerTag {
    pub tags: Option<Vec<String>>,
    pub actions: Option<Vec<String>>,
}
