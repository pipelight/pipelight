#![allow(dead_code)]
pub mod logs;
use chrono::{DateTime, Local, NaiveDateTime, Offset, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::{Result, Value};
use std::clone::Clone;
use std::cmp::PartialEq;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub pipelines: Vec<Pipeline>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Pipeline {
    pub name: String,
    pub steps: Vec<Step>,
    pub trigger: Option<Vec<Trigger>>,
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
    pub hook: VecOrString,
    pub branch: VecOrString,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum VecOrString {
    Vec,
    String,
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

pub fn type_of<T>(_: &T) -> String {
    let res = format!("{}", std::any::type_name::<T>());
    return res;
}

pub struct Path<'a> {
    pub folder: &'a str,
    pub file: &'a str,
}
