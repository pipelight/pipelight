use crate::types::Status;
use colored::Colorize;
// use convert_case::{Case, Casing};
use std::fmt;
pub use std::process::Output;

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let icon = "●";
        match *self {
            Status::Started => write!(f, "{} Started", icon),
            Status::Succeeded => write!(f, "{} {}", icon.blue(), "Succeeded".bold()),
            Status::Failed => write!(f, "{} {}", icon.red(), "Failed".normal().bold()),
            Status::Running => write!(f, "{} {}", icon.green(), "Running".bold()),
            Status::Aborted => write!(f, "{} {}", icon.yellow(), "Aborted".bold()),
        };
        Ok(())
    }
}
