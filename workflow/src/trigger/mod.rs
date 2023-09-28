// Global var
// use crate::globals::TRIGGER_ENV;

// Error Handling
use miette::{Error, IntoDiagnostic, Result};

// Globbing
use glob::Pattern;

// Types
use crate::types::Trigger;
use utils::git::{Flag, Git};

// Tests
mod test;

impl Trigger {
    /// Return actual triggering env with a modified flag
    pub fn flag(flag: Option<Flag>) -> Result<Trigger> {
        let mut env: Trigger = Trigger::default();
        if Git::new().exists() {
            env.branch = Git::new().get_branch().ok();
            env.tag = Git::new().get_tag().ok();
        }
        if flag.is_some() {
            env.action = flag;
        } else if env.action.is_none() {
            env.action = Some(Flag::default());
        }
        Ok(env)
    }
    // Success if trigger has same action or None
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
