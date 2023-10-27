// Tests
mod test;
// Structs
use crate::types::{Trigger, TriggerBranch, TriggerTag};
use utils::git::{Flag, Git};
// Global var
use crate::globals::TRIGGER_ENV;
// Error Handling
use miette::Result;

mod display;
mod getters;
mod is;

// Triggers are an essential component of pipeline automation.
// A trigger is an association of an *Action* and a *Git ref*(branch or tag).
//
// Triggering environment:
//
// When you trigger a pipeline, informations about the triggerring environment are gathered into a Trigger.
// This Trigger is then matched against the ones declared in your pipeline definition.
//
// Permissive matching Algorithm: **None -> Everything**
//
// Used on "pipelight run".
// If no restriction, pipeline can be executed.
//
//
// If no triggering action is explicitly declared in the pipeline config then
// the pipeline can be triggered on every action.
// It goes same with the branches and tag.
//
// ```
// let env = Trigger {
//   branch: "main",
//   action: manual,
// };
//
// let pipeline_trigger = Trigger {
//   branch: None,
//   action: None,
// };
//
// assert!(env.is_match(pipeline_trigger).is_ok())
// ```
// This way you could think that a single **git-push** could trigger your every pipelines.
//
// Restrictive matching Algorithm: **None -> Nothing**
//
// Used on "pipelight trigger".
// If no restriction, pipeline can not be executed.
//
// If no triggering action is explicitly declared in the pipeline config then
// the pipeline can not be triggered on any actions.
//
//

impl Trigger {
    pub fn get() -> Result<Trigger> {
        let env: Trigger = TRIGGER_ENV.lock().unwrap().clone();
        Ok(env)
    }
    pub fn set(flag: Option<Flag>) -> Result<Trigger> {
        // Get the global
        let env: Trigger;

        let mut branch = None;
        let mut tag = None;
        let mut action = None;
        // Storage value
        let mut commit = None;

        // Get git info
        if Git::new().exists() {
            branch = Git::new().get_branch().ok();
            tag = Git::new().get_tag().ok();
            commit = Git::new().get_commit().ok();
        }
        // Set env action to flag
        if flag.is_some() {
            action = flag;
        } else
        // Set env action to default
        if action.is_none() {
            action = Some(Flag::default());
        }

        // Set the global trigger
        if tag.is_some() {
            env = Trigger::TriggerTag(TriggerTag {
                tag,
                action,
                ..TriggerTag::default()
            });
            *TRIGGER_ENV.lock().unwrap() = env.clone();
        } else {
            env = Trigger::TriggerBranch(TriggerBranch {
                branch,
                action,
                ..TriggerBranch::default()
            });
            *TRIGGER_ENV.lock().unwrap() = env.clone();
        }

        Ok(env)
    }
}
