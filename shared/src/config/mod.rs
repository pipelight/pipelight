use crate::exec::subprocess::exec_attached;
use crate::types::{Config, Path, Pipeline};
use log::{debug, error, info, trace, warn};
use project_root::get_project_root;
use std::error::Error;

/// Return the config from .ts file inside the working dir.
pub fn load_config() -> Result<Config, Box<dyn Error>> {
    //Ensure config file exist
    let config_path = "./pipelight.config.ts";
    let is_exist = std::path::Path::new(config_path).exists();
    if !is_exist {
        let message = "Config file not found.";
        let hint = "Use \"pipelight init\" to generate config file or create pipelight.config.ts";
        error!("{}", message);
        debug!("{}", hint);
        return Err(Box::from(message));
    }

    let executable = "ts-node --transpile-only";
    let root = get_project_root()?;
    let to_str_result = root.to_str();
    let folder = match to_str_result {
        Some(res) => res,
        None => {
            let message = "Internal error: Couldn't find project root";
            error!("{}", message);
            return Err(Box::from(message));
        }
    };
    let path = Path {
        folder: folder,
        file: "typescript/scripts/main.ts",
    };
    let command = format!("{} {}/{}", executable, path.folder, path.file);
    let data = exec_attached(&command)?;

    // Typecast Json output
    let config_result = serde_json::from_str::<Config>(&data);

    let config = match config_result {
        Ok(res) => {
            return Ok(res);
        }
        Err(e) => {
            error!("From config file: {}", e);
            debug!("Json output:\n{}", data);
            return Err(Box::from(e));
        }
    };
}

/// Apply constraints to the Config struct
pub fn check_config(config: Config) -> Result<Config, Box<dyn Error>> {
    let names = config
        .pipelines
        .iter()
        .map(|p| &p.name)
        .cloned()
        .collect::<Vec<String>>();

    //Clone vector and remove duplicates
    let mut dedup = names.clone();
    dedup.sort();
    dedup.dedup();

    let has_duplicate = dedup.len() != names.len();

    if has_duplicate {
        let message = "Duplicate pipeline names in config";
        error!("{}", message);
        Err(Box::from(message))
    } else {
        Ok(config)
    }
}
pub fn get_config() -> Result<Config, Box<dyn Error>> {
    let mut config = load_config()?;
    config = check_config(config)?;
    Ok(config)
}
pub fn get_pipeline(name: String) -> Result<Pipeline, Box<dyn Error>> {
    let config = get_config()?;
    let pipeline_result = config
        .pipelines
        .iter()
        .filter(|p| p.name == name)
        .cloned()
        .next();
    let pipeline = match pipeline_result {
        Some(res) => return Ok(res),
        None => {
            let message = format!("Couldn't find pipeline {:?}", name);
            error!("{}", message);
            return Err(Box::from(message));
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::type_of;
    #[test]
    fn internal() {
        let res = load_config();
    }
}
