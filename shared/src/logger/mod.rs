use crate::types::Path;
use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};
use log4rs;
use project_root::get_project_root;
use std::error::Error;
pub fn set_logger() -> Result<(), Box<dyn Error>> {
    let path = Path {
        folder: get_project_root()?.to_str().unwrap().to_owned(),
        file: "shared/src/logger/log4rs.yml".into(),
    };
    let full_path = format!("{}/{}", path.folder, path.file);
    let _handle = log4rs::init_file(full_path, Default::default())?;
    Ok(())
}
