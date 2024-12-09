//! Copy of (/cast/config/types.rs).
//! Generates Types and methods for typescript deno library.

// Traits
use serde::{Deserialize, Serialize};
// Typescript
use tsify::Tsify;

/**
Options to tweak global pipelines behavior
*/
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ConfigOpts {
    // Wheteher every pipelines should be attached or detached from the standard I/O
    // when triggered by a git hook.
    pub attach: Option<bool>,
    pub log_level: Option<String>,
}

/**
The Config struct only contains an optional list of pipelines.

Yes, we could have load the pipeline list directly without the burden of encapsulate them into this struct.
But the config file isn't meant to stay as is.

It is as is to let room for top level configuration that will come after **v1.0.1**.
- boolean to declare global user pipelines accessible from everywhere in the fs.
- eventually other optional things like credentials, daemon config...

*/
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub pipelines: Option<Vec<Pipeline>>,
    pub options: Option<ConfigOpts>,
}

/**
Options to tweak pipelines behavior
*/
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PipelineOpts {
    // Wheteher the pipeline should be attached or detached from the standard I/O
    // when triggered by a git hook.
    pub attach: Option<bool>,
    pub log_level: Option<String>,
}

/**
Pipelines are a named list of steps and parallel steps.
*/
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Tsify)]
#[serde(deny_unknown_fields)]
#[tsify(from_wasm_abi)]
pub struct Pipeline {
    pub name: String,
    pub triggers: Option<Vec<Trigger>>,
    pub steps: Vec<StepOrParallel>,
    #[serde(flatten)]
    pub fallback: Option<Fallback>,
    pub options: Option<PipelineOpts>,
}

/**
Options to tweak step behavior and command execution
*/
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct StepOpts {
    // The step's command execution behavior
    pub mode: Option<String>,
}

/**
Steps are a named list of Commands.
*/
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Step {
    pub name: String,
    pub commands: Vec<String>,
    pub options: Option<StepOpts>,
    #[serde(flatten)]
    pub fallback: Option<Fallback>,
}

/**
Parallel are unnamed list of steps.
*/
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Parallel {
    pub parallel: Vec<Step>,
    // pub options: Option<StepOpts>,
    pub mode: Option<String>,
    #[serde(flatten)]
    pub fallback: Option<Fallback>,
}

/**
The StepOrParallel enum is a conveninent enum designed for
a pipeline to accept either steps and parallel steps.
As a  developer, I find it a bit of an overhead to use.
But it is the simplest way I have found to make
a usable **Union** (Step must be This type OR This type).
*/
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
pub enum StepOrParallel {
    Step(Step),
    Parallel(Parallel),
}

/**
A pipeline and a step can have fallbacks.
They are steps to be triggered on specific events.
For example if a pipeline fails and if its on_failure fallback is defined
the on_failure fallback is triggered.
*/
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Fallback {
    pub on_started: Option<Vec<StepOrParallel>>,
    pub on_failure: Option<Vec<StepOrParallel>>,
    pub on_success: Option<Vec<StepOrParallel>>,
    pub on_abortion: Option<Vec<StepOrParallel>>,
}

/**
Triggers are casted into multiple types.
*/
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
pub enum Trigger {
    TriggerBranch(TriggerBranch),
    TriggerTag(TriggerTag),
}

/**
A trigger that is a combination of actions over a git branch.
*/
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct TriggerBranch {
    pub branches: Option<Vec<String>>,
    pub actions: Option<Vec<String>>,
}

/**
A trigger that is a combination of actions over a git tag.
*/
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Tsify)]
#[serde(deny_unknown_fields)]
#[tsify(from_wasm_abi)]
pub struct TriggerTag {
    pub tags: Option<Vec<String>>,
    pub actions: Option<Vec<String>>,
}
