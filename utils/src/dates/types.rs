use serde::{Deserialize, Serialize};

/**
The pipelight simple duration object that abstract standart and chrono crates
duration usage.
Implements convenient methods.
*/
#[derive(Default, Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Duration {
    // iso8601 date string
    pub started_at: Option<String>,
    // iso8601 date string
    pub ended_at: Option<String>,
    // iso8601 duration string
    #[serde(skip)]
    pub computed: Option<String>,
}
