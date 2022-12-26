use crate::exec::subprocess::{exec, exec_attached, exec_detached};
use crate::types::config::{Config, Pipeline};
use crate::types::Path;
use log::{debug, error, info, trace, warn};
use project_root::get_project_root;
use std::error::Error;

/// Ensure file exist
fn config_exist() -> Result<(), Box<dyn Error>> {
    let config_path = "./pipelight.config.ts";
    let exist = std::path::Path::new(config_path).exists();
    if exist {
        Ok(())
    } else {
        let message = "Config file not found.";
        let hint =
            "Use \"pipelight init\" to generate config file\n or move to the right directory";
        error!("{}", message);
        debug!("{}", hint);
        Err(Box::from(message))
    }
}

/// Return project root path as string
fn get_root() -> Result<String, Box<dyn Error>> {
    let root = get_project_root()?;
    let to_str_result = root.to_str();
    let path = match to_str_result {
        Some(res) => return Ok(res.to_owned()),
        None => {
            let message = "Internal error: Couldn't find project root";
            warn!("{}", message);
            return Err(Box::from(message));
        }
    };
}

/// Lint ts files.
pub fn lint_config() -> Result<(), Box<dyn Error>> {
    config_exist()?;
    let executable = "tsc --noEmit";
    let file = "pipelight.config.ts";
    let command = format!("{} {}", executable, file);
    info!("Linting config file");
    let res = exec(&command)?;

    if res.status {
        info!("Config file ok");
    } else {
        warn!("Config file contains errors");
        println!("{}", res.stdout.unwrap());
        println!("{}", res.stderr.unwrap());
    }
    Ok(())
}

/// Return the config from .ts file inside the working dir.
fn load_config() -> Result<Config, Box<dyn Error>> {
    //Ensure config file exist
    let executable = "ts-node --transpile-only";
    let path = Path {
        folder: &get_root()?,
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
            warn!("From config file: {}", e);
            debug!("Json output:\n{}", data);
            return Err(Box::from(e));
        }
    };
}

/// Apply constraints to the Config struct
fn check_config(config: Config) -> Result<Config, Box<dyn Error>> {
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
        warn!("{}", message);
        Err(Box::from(message))
    } else {
        Ok(config)
    }
}
pub fn get_config() -> Result<Config, Box<dyn Error>> {
    config_exist()?;
    let mut config = load_config()?;
    config = check_config(config)?;
    Ok(config)
}
pub fn get_pipeline(name: &str) -> Result<Pipeline, Box<dyn Error>> {
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
            warn!("{}", message);
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
