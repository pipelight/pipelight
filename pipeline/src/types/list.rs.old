use crate::cast::Config;
use crate::types::Pipelines;
use log::info;
use std::error::Error;

impl Pipelines {
    pub fn list() -> Result<(), Box<dyn Error>> {
        let config = Config::new()?;
        let pipelines_from_logs = Pipelines::get_logged()?;
        for pipeline in &config.pipelines.unwrap() {
            info!(target: "nude","pipeline_name\n");
            let names: Vec<String> = pipelines_from_logs.iter().map(|e| e.clone().name).collect();
            println!("{}", pipeline.name);
            if names.contains(&pipeline.name) {
                // retrieve last run and get info from logs
                // println!(
                //     "{0: <10} {1: <20} {2: <10} {3}",
                //     "status", "last_run_date", "hook", "name"
                // );
            }
        }
        Ok(())
    }
}
