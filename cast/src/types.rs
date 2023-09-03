use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Fallback {
    pub on_started: Option<Vec<StepOrParallel>>,
    pub on_failure: Option<Vec<StepOrParallel>>,
    pub on_success: Option<Vec<StepOrParallel>>,
    pub on_abortion: Option<Vec<StepOrParallel>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub pipelines: Option<Vec<Pipeline>>,
}
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Pipeline {
    pub name: String,
    pub triggers: Option<Vec<Trigger>>,
    pub steps: Vec<StepOrParallel>,
    #[serde(flatten)]
    pub fallback: Option<Fallback>,
}
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Step {
    pub name: String,
    pub commands: Vec<String>,
    pub mode: Option<String>,
    #[serde(flatten)]
    pub fallback: Option<Fallback>,
}
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Parallel {
    pub parallel: Vec<Step>,
    pub mode: Option<String>,
    #[serde(flatten)]
    pub fallback: Option<Fallback>,
}
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
pub enum StepOrParallel {
    Step(Step),
    Parallel(Parallel),
}
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
pub enum Trigger {
    TriggerBranch(TriggerBranch),
    TriggerTag(TriggerTag),
}
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct TriggerBranch {
    pub branches: Option<Vec<String>>,
    pub actions: Option<Vec<String>>,
}
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct TriggerTag {
    pub tags: Option<Vec<String>>,
    pub actions: Option<Vec<String>>,
}
