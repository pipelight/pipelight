// Structs
use crate::git::Git;
// Error Handling
use miette::{Error, IntoDiagnostic, Result};

impl Git {
    /**
    Returns a boolean, whether a repository exists or not.
    */
    pub fn exists(self) -> bool {
        self.repo.is_some()
    }
    /**
    Returns the checkout branch
    */
    pub fn get_branch(&self) -> Result<String> {
        // Only tested on attached HEAD
        // No edge case when head is a commit or else...
        let repo = self.repo.as_ref().unwrap();
        let head = repo.head().into_diagnostic()?;
        let name = head.shorthand().unwrap().to_owned();
        Ok(name)
    }
    /**
    Returns the tag if the head is a tag or if the latest commit is a tag
    */
    pub fn get_tag(&self) -> Result<String> {
        let repo = self.repo.as_ref().unwrap();
        let head = repo.head().into_diagnostic()?;
        if head.is_tag() {
            let tag = head.name().unwrap().to_string();
            Ok(tag)
        } else {
            Err(Error::msg("The current HEAD is not a tag"))
        }
    }
    /**
    Returns the latest commit or the checkout commit
    */
    pub fn get_commit(&self) -> Result<String> {
        let repo = self.repo.as_ref().unwrap();
        let head = repo.head().into_diagnostic()?;
        let commit_id = head.peel_to_commit().into_diagnostic()?.id().to_string();
        Ok(commit_id)
    }
}
