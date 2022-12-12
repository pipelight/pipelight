use crate::exec::exec_attach;
use crate::types::{Config, Path};
use log::{debug, error, info, trace, warn};
use project_root::get_project_root;
use std::error::Error;

/// Return the config from .ts file inside the working dir.
pub fn load_config() -> Result<Config, Box<dyn Error>> {
    let executable = "ts-node";
    let path = Path {
        folder: get_project_root()?.to_str().unwrap().to_owned(),
        file: "typescript/main.ts".into(),
    };
    let command = format!("{} {}/{}", executable, path.folder, path.file);
    let data = exec_attach(command)?;

    // Typecast Json output
    let config_result = serde_json::from_str::<Config>(&data);

    let config = match config_result {
        Ok(res) => {
            debug!("{:#?}", res);
            return Ok(res);
        }
        Err(e) => {
            error!("{}", data);
            println!("{}", e);
            println!("{}", data);
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

    //Vlone vector and emove duplicates
    let mut dedup = names.clone();
    dedup.sort();
    dedup.dedup();

    //Compare bath vecors
    let has_duplicate = dedup.len() != names.len();

    trace!("{}", has_duplicate);
    debug!("{:?}", names);

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::type_of;
    use std::env;
    use std::path::Path;
    #[test]
    fn internal() {
        let root = Path::new("./test");
        assert!(env::set_current_dir(&root).is_ok());
        println!(
            "Successfully changed working directory to {:#?}!",
            root.display()
        );
        // set cwd to test in other folder
        let res = load_config();
    }
}
