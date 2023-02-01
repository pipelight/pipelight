// Types that exist just to so json_serde can translate json into usable... things
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
mod config;
mod default;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(skip)]
    pub file: String,
    pub pipelines: Option<Vec<Pipeline>>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Pipeline {
    pub name: String,
    pub steps: Vec<StepOrParallel>,
    pub triggers: Option<Vec<Trigger>>,
    pub on_failure: Option<Vec<String>>,
    pub on_success: Option<Vec<String>>,
    pub on_abortion: Option<Vec<String>>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Step {
    pub non_blocking: Option<bool>,
    pub name: String,
    pub commands: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Parallel {
    pub non_blocking: Option<bool>,
    pub parallel: Vec<Step>,
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
pub struct Trigger {
    pub actions: Option<Vec<String>>,
    pub branches: Vec<String>,
}
