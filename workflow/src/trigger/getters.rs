// Struct
use crate::types::Trigger;
use utils::git::Flag;
// Error Handling
use miette::Result;

impl Trigger {
    pub fn get_action(&self) -> Result<Option<Flag>> {
        match self {
            Trigger::TriggerBranch(self_trigger_branch) => Ok(self_trigger_branch.action.clone()),
            Trigger::TriggerTag(self_trigger_tag) => Ok(self_trigger_tag.action.clone()),
        }
    }
    pub fn get_commit(&self) -> Result<Option<String>> {
        match self {
            Trigger::TriggerBranch(self_trigger_branch) => Ok(self_trigger_branch.commit.clone()),
            Trigger::TriggerTag(self_trigger_tag) => Ok(self_trigger_tag.commit.clone()),
        }
    }
}
