// Test
mod test;
// Globals
use crate::globals::CONFIG;
// Error Handling
use miette::Result;
// Struct
use crate::pipeline::Filters;
use crate::types::Config;
use utils::git::{Flag, Special};

impl Config {
    pub fn get() -> Result<Self> {
        let config = CONFIG.lock().unwrap().clone();
        Ok(config)
    }
    pub fn has_watch_flag(&self) -> Result<bool> {
        if let Some(pipelines) = self.pipelines.clone() {
            let mut pipelines = Filters::to_hashmap(pipelines);
            pipelines.retain(|_, pipeline| {
                if let Some(triggers) = pipeline.triggers.clone() {
                    triggers
                        .iter()
                        .any(|e| e.action == Some(Flag::Special(Special::Watch)))
                } else {
                    false
                }
            });
            return Ok(!pipelines.is_empty());
        }
        Ok(false)
    }
}
