/// Strict trigger matching algorithm
// Test
mod test;
// Struct
use crate::types::{Trigger, TriggerBranch, TriggerTag};
// Error Handling
use miette::{Error, IntoDiagnostic, Result};
// Globbing
use glob::Pattern;

impl Trigger {
    pub fn has_match_strict(&self, list: Vec<Self>) -> Result<bool> {
        match self {
            // If self trigger and trigger in list are same enum type
            // then compare
            Trigger::TriggerTag(self_trigger_tag) => {
                for trigger in list {
                    let is = match trigger {
                        Trigger::TriggerTag(trigger_tag) => {
                            self_trigger_tag.is_match_strict(&trigger_tag)
                        }
                        _ => Ok(false),
                    };
                    if is? {
                        return Ok(true);
                    }
                }
            }
            Trigger::TriggerBranch(self_trigger_branch) => {
                for trigger in list {
                    let is = match trigger {
                        Trigger::TriggerBranch(trigger_branch) => {
                            self_trigger_branch.is_match_strict(&trigger_branch)
                        }
                        _ => Ok(false),
                    };
                    if is? {
                        return Ok(true);
                    }
                }
            }
        };
        Ok(false)
    }
}
impl TriggerBranch {
    pub fn is_match_strict(&self, trigger: &Self) -> Result<bool> {
        Ok(self.is_action_match_strict(trigger)? && self.is_branch_match_strict(trigger)?)
    }
    fn is_action_match_strict(&self, trigger: &Self) -> Result<bool> {
        if trigger.action.is_some() && self.action.is_some() && trigger.action == self.action {
            Ok(true)
        } else {
            Ok(false)
        }
    }
    fn is_branch_match_strict(&self, trigger: &Self) -> Result<bool> {
        // If the project is not a git repo
        // OR
        // If the pipeline has no defined triggering branch
        if trigger.branch.is_none() || self.branch.is_none() {
            return Ok(true);
        }
        // Globbing pattern matching
        let glob = Pattern::new(&trigger.branch.clone().unwrap()).into_diagnostic()?;
        let glob_match = glob.matches(&self.clone().branch.unwrap());
        if glob_match {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
impl TriggerTag {
    pub fn is_match_strict(&self, trigger: &Self) -> Result<bool> {
        Ok(self.is_action_match_strict(trigger)? && self.is_tag_match_strict(trigger)?)
    }
    /**
    Return success if trigger has same action or None
    */
    pub fn is_action_match_strict(&self, trigger: &Self) -> Result<bool> {
        if trigger.action.is_some() && self.action.is_some() && trigger.action == self.action {
            Ok(true)
        } else {
            Ok(false)
        }
    }
    pub fn is_tag_match_strict(&self, trigger: &Self) -> Result<bool> {
        // If the project is not a git repo
        // OR
        // If the pipeline has no defined triggering tag
        if trigger.tag.is_none() || self.tag.is_none() {
            return Ok(true);
        }
        // Globbing pattern matching
        let glob = Pattern::new(&trigger.tag.clone().unwrap()).into_diagnostic()?;
        let glob_match = glob.matches(&self.clone().tag.unwrap());
        if glob_match {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
