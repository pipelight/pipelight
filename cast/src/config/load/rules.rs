use crate::Config;

// Error Handling
use log::warn;
use miette::Result;

impl Config {
    /**
     * Enforce pipelines rules
     * - No whitespaces in pipeline names
     *
     */
    pub fn strict_check(&mut self) -> Result<Config> {
        if let Some(pipelines) = self.pipelines.clone() {
            for pipeline in pipelines {
                if pipeline.name.contains(char::is_whitespace) {
                    warn!("The pipeline {} has an invalide name", pipeline.name);
                    println!("{:#?}", pipeline);
                }
            }
        }
        Ok(self.to_owned())
    }
}
