use super::Config;
use utils::get_root;

impl Config {
    pub fn new() -> Result<Config, Box<dyn Error>> {
        let default = Config {}

        Git::new();
        Ok(Config::get()?)
    }
    /// Ensure file exist
    pub fn get() -> Result<Config, Box<dyn Error>> {
        Config::exists()?;
        let mut config = Config::load()?;
        config = Config::check(&config)?;
        Ok(config)
    }
    /// Lint ts files.
    pub fn lint(&self) -> Result<(), Box<dyn Error>> {
        Config::exists()?;
        let executable = "tsc --noEmit";
        let file = "pipelight.config.ts";
        let command = format!("{} {}", executable, file);
        info!("Linting config file");
        let res = Exec::new().simple(&command)?;

        if res.status {
            info!("Config file ok");
        } else {
            warn!("Config file contains errors");
            println!("{}", res.stdout.unwrap());
            println!("{}", res.stderr.unwrap());
        }
        Ok(())
    }
    pub fn pipeline(&self, name: &str) -> Result<Pipeline, Box<dyn Error>> {
        let pipeline_result = &self
            .pipelines
            .iter()
            .filter(|p| p.name == name)
            .cloned()
            .next();
        let pipeline = match pipeline_result {
            Some(res) => return Ok(res.to_owned()),
            None => {
                let message = format!("Couldn't find pipeline {:?}", name);
                warn!("{}", message);
                return Err(Box::from(message));
            }
        };
    }
    fn exists() -> Result<(), Box<dyn Error>> {
        let path = Path::new("./pipelight.config.ts");
        let exist = Path::new(path).exists();
        if exist {
            Ok(())
        } else {
            let message = "Config file not found.";
            let hint =
                "Use \"pipelight init\" to generate config file\n or move to the right directory";
            warn!("{}", message);
            debug!("{}", hint);
            Err(Box::from(message))
        }
    }

    /// Return the config from .ts file inside the working dir.
    fn load() -> Result<Config, Box<dyn Error>> {
        //Ensure config file exist
        let executable = "ts-node --transpile-only";
        let folder = &get_root()?;
        let file = "typescript/scripts/main.ts";
        let command = format!("{} {}/{}", executable, folder, file);
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
    fn check(config: &Config) -> Result<Self, Box<dyn Error>> {
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
            Ok(config.to_owned())
        }
    }
}

