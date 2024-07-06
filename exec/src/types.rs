use crate::io::Io;
use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::cmp::PartialEq;
use pipelight_utils::dates::Duration;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct SelfProcess;
