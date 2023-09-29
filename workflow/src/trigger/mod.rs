// Tests
mod test;
// Structs
use crate::types::Trigger;
use utils::git::{Flag, Git};
// Global var
use crate::globals::TRIGGER_ENV;
// Error Handling
use miette::Result;

mod is;

impl Trigger {
    /**
    Return the computed triggering env when flag set to None.
    or
    Return the modified triggering env when flag is set.
    */
    pub fn flag(flag: Option<Flag>) -> Result<Trigger> {
        // Get the global
        let mut env: Trigger = TRIGGER_ENV.lock().unwrap().clone();

        if Git::new().exists() {
            env.branch = Git::new().get_branch().ok();
            env.tag = Git::new().get_tag().ok();
        }
        if flag.is_some() {
            env.action = flag;
        } else if env.action.is_none() {
            env.action = Some(Flag::default());
        }

        // Set the global
        *TRIGGER_ENV.lock().unwrap() = env.clone();
        Ok(env)
    }
}
