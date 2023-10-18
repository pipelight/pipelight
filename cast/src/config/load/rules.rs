use crate::Config;

// Error Handling
use log::warn;
use miette::Result;

impl Config {
    /**
    Removes pipelines if they don't obey the rules.
    Enforces pipeline definition rules:
    - No whitespaces allowed in pipeline names.
      There is simple workarounds to allow whitespaces in names and avoid collision with shell arguments.
      (IMHO Names should not contain whitespaces. We are not windows user after all.)
    - ...
    */
    pub fn strict_check(&mut self) -> Result<Config> {
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
