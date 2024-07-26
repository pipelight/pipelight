use crate::dates::Duration;
use crate::exec::*;
use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::cmp::PartialEq;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct SelfProcess;
