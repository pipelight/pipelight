// Error Handling
use miette::{Error, Result};

// Tests
mod test;

pub fn std_duration_to_iso8601(duration: &std::time::Duration) -> Result<String> {
    let chrono_duration = chrono::Duration::from_std(duration.to_owned()).ok();
    if let Some(chrono_duration) = chrono_duration {
        let duration_iso_8601 = format!("{}", chrono_duration);
        Ok(duration_iso_8601)
    } else {
        Err(Error::msg("Bad std::Duration instance"))
    }
}
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
