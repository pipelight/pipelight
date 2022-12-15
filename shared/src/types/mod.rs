#![allow(dead_code)]
pub mod logs;
use chrono::{DateTime, Local, NaiveDateTime, Offset, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::{Result, Value};
use std::clone::Clone;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub pipelines: Vec<Pipeline>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pipeline {
    pub name: String,
    pub steps: Vec<Step>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Step {
    pub name: String,
    pub commands: Vec<String>,
    pub on_failure: Option<Vec<String>>,
}

pub fn type_of<T>(_: &T) -> String {
    let res = format!("{}", std::any::type_name::<T>());
    return res;
}

pub struct Path {
    pub folder: String,
    pub file: String,
}
