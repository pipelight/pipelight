// Structs
use crate::git::Git;
// Error Handling
use miette::{Error, IntoDiagnostic, Result};

impl Git {
    /**
     * Returns a boolean, whether a repository exists or not.
     */
    pub fn exists(self) -> bool {
        self.repo.is_some()
    }
    /**
     * Returns the checkout branch
     */
    pub fn get_branch(&self) -> Result<String> {
        // Only tested on attached HEAD
        // No edge case when head is a commit or else...
        let repo = self.repo.as_ref().unwrap();
        let head = repo.head_ref().into_diagnostic()?;
        match head {
            Some(x) => Ok(x.name().shorten().to_string()),
            None => Err(Error::msg("Repo is in detached HEAD state")),
        }
    }
    /**
     * Returns the tag if the head is a tag or if the latest commit is a tag
     */
    pub fn get_tag(&self) -> Result<String> {
        let repo = self.repo.as_ref().unwrap();
        let head = repo.head_ref().into_diagnostic()?;
        if let Some(mut head) = head {
            let head_commit_id = head.peel_to_commit().into_diagnostic()?.id();

            // Get every tags
            let refs = repo.references().into_diagnostic()?;
            let tags = refs.tags().into_diagnostic()?;

            // Get name of tags pointing to HEAD
            let mut head_tags: Vec<String> = tags
                // Safe unwrap Result<Reference>
                .filter(|x| x.is_ok())
                .map(|x| x.unwrap())
                // Does tag point to HEAD
                .filter(|x| x.clone().peel_to_commit().unwrap().id() == head_commit_id)
                // Short name
                .map(|x| x.name().shorten().to_string())
                .collect();

            head_tags.reverse();
            if let Some(latest_tag) = head_tags.first() {
                return Ok(latest_tag.to_owned());
            }
        } else {
            return Err(Error::msg("Repo is in detached HEAD state"));
        }

        Err(Error::msg("The current HEAD is not a tag"))
    }
    /**
     * Returns the latest commit or the checkout commit
     */
    pub fn get_commit(&self) -> Result<String> {
        let repo = self.repo.as_ref().unwrap();
        let mut head = repo.head().into_diagnostic()?;
        let commit_id = head
            .peel_to_commit_in_place()
            .into_diagnostic()?
            .id()
            .to_string();
        Ok(commit_id)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_repo_branch() -> Result<()> {
        let res = Git::new().get_branch()?;
        println!("{}", res);
        Ok(())
    }
    #[test]
    fn get_repo_tag() -> Result<()> {
        let res = Git::new().get_tag()?;
        println!("{}", res);
        Ok(())
    }
}
