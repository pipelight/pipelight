use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::time;
#[derive(Default, Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Duration {
    // iso8601 DateTime
    pub started_at: Option<String>,
    pub ended_at: Option<String>,
    // iso8601 Duration
    #[serde(skip)]
    pub computed: Option<String>,
}
