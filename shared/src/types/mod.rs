#![allow(dead_code)]
pub mod config;
pub mod logs;
use chrono::{DateTime, Local, NaiveDateTime, Offset, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::{Result, Value};
use std::clone::Clone;
use std::cmp::PartialEq;

pub struct Path<'a> {
    pub folder: &'a str,
    pub file: &'a str,
}
