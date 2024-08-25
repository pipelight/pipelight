use crate::Config;

// Error Handling
use log::warn;
use miette::Result;
use pipelight_error::PipelightError;

impl Config {
    /**
    Raises a warning.

    Enforces pipeline definition rules:
    - No whitespaces allowed in pipeline names.
    */
    pub fn strict_check(&mut self) -> Result<Config, PipelightError> {
        if let Some(pipelines) = self.pipelines.clone() {
            for pipeline in pipelines {
                if pipeline.name.contains(char::is_whitespace) {
                    warn!("The pipeline \"{}\" contains whitespaces", pipeline.name);
                }
            }
        }
        Ok(self.to_owned())
    }
}
