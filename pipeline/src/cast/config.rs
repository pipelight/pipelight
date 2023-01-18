use super::Config;
use exec::Exec;
use log::{debug, error, warn};

// standard lib
use std::env::current_dir;
use std::error::Error;
use std::path::Path;
use std::process::exit;

impl Config {
    pub fn get() -> Config {
        let file = "pipelight.config.mjs".to_owned();
        return Config::load(&file).unwrap();
    }
    /// Ensure config file exists
    fn exists(file: &String) -> Result<bool, Box<dyn Error>> {
        let pwd = current_dir().unwrap();
        let string = format!("{}/{}", &pwd.display().to_string(), file);
        let path = Path::new(&string);
        let exist = Path::new(path).exists();
        if !exist {
            let message = "Config file not found.";
            error!("{}", message);
            exit(1);
        } else {
            return Ok(exist);
        }
    }

    /// Return the config from .mjs file inside the working dir.
    fn load(file: &String) -> Result<Config, Box<dyn Error>> {
        Config::exists(file)?;
        let pwd = current_dir().unwrap();
        let string = format!("{}/{}", &pwd.display().to_string(), file);
        let path = Path::new(&string);

        let executable = "node -e";
        // Javascript with rust escape sequence (double curly braces)
        let script = format!(
            "\'
            const stock = console;
            console = {{}};
            const promess = import(`{}`);
            promess
              .then((res) => {{ 
                console = stock;
                const config = res.default;
                const json = JSON.stringify(config, null, 2);
                console.log(json);
              }})
              .catch((err) => {{
                console.log(err);
              }});
        \'",
            path.display().to_string()
        );
        let command = format!("{} {}", executable, script);
        let data = Exec::new().attached(&command)?;
        let config_result = serde_json::from_str::<Config>(&data);
        match config_result {
            Ok(res) => {
                return Ok(res);
            }
            Err(e) => {
                let message = format!("From config file: {}", e);
                warn!("{}", message);
                debug!("Json output:\n{}", data);
                return Err(Box::from(message));
            }
        };
    }
}
