use crate::Config;

// Error Handling
use log::warn;
use miette::{Error, Result};
use pipelight_error::{LibError, PipelightError};

impl Config {
    /**
     * Enforces pipeline definition rules:
     */
    pub fn strict_check(&mut self) -> Result<Config, PipelightError> {
        self.strict_whitespace()
    }
    /**
     * No whitespaces allowed in pipeline names.
     */
    pub fn strict_whitespace(&mut self) -> Result<Config, PipelightError> {
        if let Some(pipelines) = self.pipelines.clone() {
            let mut help = "Replace whitespaces with underscores(_):\n\n".to_owned();

            // Cycle through pipelines
            let mut has_whitespace = false;
            for pipeline in pipelines {
                if pipeline.name.contains(char::is_whitespace) {
                    has_whitespace = true;
                    let name = pipeline.name.clone();
                    let hint = name.replace(" ", "_");
                    help += &format!("{} -> {}\n", name, hint);
                }
            }
            if has_whitespace {
                let e = LibError {
                    message: "A pipeline name can't contain whitespaces.".to_owned(),
                    help,
                };
                return Err(e.into());
            }
        }
        Ok(self.to_owned())
    }
}
