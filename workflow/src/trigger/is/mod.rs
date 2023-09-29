// Test
mod test;
// Struct
use crate::types::Trigger;
// Error Handling
use miette::{Error, IntoDiagnostic, Result};
// Globbing
use glob::Pattern;

impl Trigger {
    /**
    Success if trigger has same action or None
    */
    pub fn is_action_match(&self, trigger: Trigger) -> Result<()> {
        if trigger.action.is_none() || trigger.action == self.action {
            Ok(())
        } else {
            let message = "no action match";
            Err(Error::msg(message))
        }
    }
    pub fn is_branch_match(&self, trigger: Trigger) -> Result<()> {
        if trigger.branch.is_none() {
            Ok(())
        } else {
            // Globbing pattern matching
            let glob = Pattern::new(&trigger.branch.unwrap()).into_diagnostic()?;
            let glob_match = glob.matches(&self.clone().branch.unwrap());
            if glob_match {
                Ok(())
            } else {
                let message = "no branch match";
                Err(Error::msg(message))
            }
        }
    }
    pub fn is_tag_match(&self, trigger: Trigger) -> Result<()> {
        if trigger.tag.is_none() || self.tag.is_none() {
            Ok(())
        } else {
            // Globbing pattern matching
            let glob = Pattern::new(&trigger.tag.unwrap()).into_diagnostic()?;
            let glob_match = glob.matches(&self.clone().tag.unwrap());
            if glob_match {
                Ok(())
            } else {
                let message = "no tag match";
                Err(Error::msg(message))
            }
        }
    }
    pub fn is_match(&self, list: Vec<Trigger>) -> Result<()> {
        for trigger in list {
            let binding = trigger.clone();
            if self.is_action_match(binding.clone()).is_ok()
                && self.is_branch_match(binding.clone()).is_ok()
                && self.is_tag_match(binding).is_ok()
            {
                return Ok(());
            }
        }
        let message = "no match";
        Err(Error::msg(message))
    }
}
