#![allow(dead_code)]
pub mod config;
pub mod logs;
use chrono::{DateTime, Local, NaiveDateTime, Offset, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::{Result, Value};
use std::clone::Clone;
use std::cmp::PartialEq;

pub fn type_of<T>(_: &T) -> String {
    let res = format!("{}", std::any::type_name::<T>());
    return res;
}

pub struct Path<'a> {
    pub folder: &'a str,
    pub file: &'a str,
}
