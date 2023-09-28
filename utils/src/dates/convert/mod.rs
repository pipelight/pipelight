// Tests
mod test;
// Structs
use crate::dates::types::Duration;
// Error Handling
use miette::{Error, Result};

/**
Convert a pipelight Duration into ISO8601 duration string
*/
impl From<&Duration> for String {
    fn from(e: &Duration) -> Self {
        let mut e = e.clone();
        let std = e.get().unwrap();
        std_duration_to_iso8601(&std).unwrap()
    }
}

/**
Convert an ISO8601 duration string into pipelight Duration
*/
impl From<&String> for Duration {
    fn from(e: &String) -> Self {
        Duration {
            started_at: None,
            ended_at: None,
            computed: Some(e.to_owned()),
        }
    }
}

/**
Convert the standard duration struct(std::time::Duration)
into an ISO8601 duration string
*/
pub fn std_duration_to_iso8601(duration: &std::time::Duration) -> Result<String> {
    let chrono_duration = chrono::Duration::from_std(duration.to_owned()).ok();
    if let Some(chrono_duration) = chrono_duration {
        let duration_iso_8601 = format!("{}", chrono_duration);
        Ok(duration_iso_8601)
    } else {
        Err(Error::msg("Bad std::Duration instance"))
    }
}

/**
The reciprocal:
Convert an ISO8601 duration string
into the standard duration struct(std::time::Duration)
*/
pub fn iso8601_to_std_duration(duration: &str) -> Result<std::time::Duration> {
    let chrono_duration: Option<iso8601_duration::Duration> = duration.parse().ok();
    if let Some(chrono_duration) = chrono_duration {
        let std_duration = chrono_duration.to_std();
        if let Some(std_duration) = std_duration {
            return Ok(std_duration);
        }
    }
    Err(Error::msg("Couldn't parse duration: Bad iso8601 duration"))
}
