// Struct
use crate::types::Trigger;
// Colors
pub use colored::control::set_override;
use colored::Colorize;
// Traits
use std::fmt;

impl fmt::Display for Trigger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // set_override(true);
        let mut string = "".to_owned();
        match self {
            Trigger::TriggerTag(self_trigger_tag) => {
                if let Some(tag) = self_trigger_tag.clone().tag {
                    string += "tag: ";
                    string += &tag;
                    string += " ";
                }
            }
            Trigger::TriggerBranch(self_trigger_branch) => {
                if let Some(branch) = self_trigger_branch.clone().branch {
                    string += "branch: ";
                    string += &branch;
                    string += " ";
                }
            }
        };
        if let Some(action) = self.get_action().unwrap() {
            string += "action: ";
            string += &String::from(&action);
            string += " ";
        }
        string = format!("[{}]", string);
        write!(f, "{}", string.white())
    }
}
