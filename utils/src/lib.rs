use project_root::get_project_root;
use std::error::Error;
pub mod git;
pub mod logger;

/// Return project root path as string
pub fn get_root() -> Result<String, Box<dyn Error>> {
    let root = get_project_root()?;
    let to_str_result = root.to_str();
    match to_str_result {
        Some(res) => return Ok(res.to_owned()),
        None => {
            let message = "Internal error: Couldn't find project root";
            // warn!("{}", message);
            return Err(Box::from(message));
        }
    };
}
