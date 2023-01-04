// Types that exist just to so json_serde can translate json into usable... things
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

mod config;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub pipelines: Option<Vec<Pipeline>>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Pipeline {
    pub name: String,
    pub steps: Vec<Step>,
    pub triggers: Option<Vec<Trigger>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Step {
    pub name: String,
    pub commands: Vec<String>,
    pub non_blocking: Option<bool>,
    pub on_failure: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Trigger {
    pub actions: Vec<String>,
    pub branches: Vec<String>,
}

pub const GIT_HOOKS: [&str; 17] = [
    "applypatch-msg",
    "pre-applypatch",
    "post-applypatch",
    "pre-commit",
    "prepare-commit-msg",
    "commit-msg",
    "post-commit",
    "pre-rebase",
    "post-checkout",
    "post-merge",
    "pre-receive",
    "update",
    "post-receive",
    "post-update",
    "pre-auto-gc",
    "post-rewrite",
    "pre-push",
];
