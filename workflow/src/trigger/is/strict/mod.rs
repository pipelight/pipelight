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
                    let is: Result<bool> = match trigger {
                        Trigger::TriggerTag(trigger_tag) => {
                            Ok(self_trigger_tag.is_match_strict(&trigger_tag)?)
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
                    let is: Result<bool> = match trigger {
                        Trigger::TriggerBranch(trigger_branch) => {
                            Ok(self_trigger_branch.is_match_strict(&trigger_branch)?)
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
        Ok(self.is_action_match_strict(trigger)? && self.is_branch_match(trigger)?)
    }
    fn is_action_match_strict(&self, trigger: &Self) -> Result<bool> {
        if trigger.action.is_some() && self.action.is_some() && trigger.action == self.action {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
impl TriggerTag {
    pub fn is_match_strict(&self, trigger: &Self) -> Result<bool> {
        Ok(self.is_action_match_strict(trigger)? && self.is_tag_match(trigger)?)
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
}
