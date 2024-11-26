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
        let head = repo.head_ref().into_diagnostic()?;
        if let Some(head) = head {
            let name = head.name().shorten().to_string();
            Ok(name)
        } else {
            Err(Error::msg("Repo is in detached HEAD state"))
        }
    }
    /**
    Returns the tag if the head is a tag or if the latest commit is a tag
    */
    pub fn get_tag(&self) -> Result<String> {
        // let repo = self.repo.as_ref().unwrap();
        // let head = repo
        //     .head_ref()
        //     .into_diagnostic()?
        //     .unwrap()
        //     .follow_to_object()
        //     .into_diagnostic()?
        //     .object()
        //     .into_diagnostic()?
        //     .to_tag_ref()
        //     .name
        //     .to_string();
        // Ok(head)
        // if let Some(mut head) = head {
        // match head.to_tag_ref() {
        //     Ok(x) => return Ok(x.decode().into_diagnostic()?.name.to_string()),
        //     Err(e) => {
        //         return Err(Error::msg("The current HEAD is not a tag"));
        //     }
        // };
        // } else {
        //     Err(Error::msg("Repo is in detached HEAD state"))
        // }
        Ok("null".to_string())
    }
    /**
    Returns the latest commit or the checkout commit
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
        let res = Git::new().get_tag();
        println!("{:#?}", res);
        Ok(())
    }
}

// /**
// Returns a boolean, whether a repository exists or not.
// */
// pub fn exists(self) -> bool {
//     self.repo.is_some()
// }
// /**
// Returns the checkout branch
// */
// pub fn get_branch(&self) -> Result<String> {
//     // Only tested on attached HEAD
//     // No edge case when head is a commit or else...
//     let repo = self.repo.as_ref().unwrap();
//     let head = repo.head().into_diagnostic()?;
//     let name = head.shorthand().unwrap().to_owned();
//     Ok(name)
// }
// /**
// Returns the tag if the head is a tag or if the latest commit is a tag
// */
// pub fn get_tag(&self) -> Result<String> {
//     let repo = self.repo.as_ref().unwrap();
//     let head = repo.head().into_diagnostic()?;
//     if head.is_tag() {
//         let tag = head.name().unwrap().to_string();
//         Ok(tag)
//     } else {
//         Err(Error::msg("The current HEAD is not a tag"))
//     }
// }
// /**
// Returns the latest commit or the checkout commit
// */
// pub fn get_commit(&self) -> Result<String> {
//     let repo = self.repo.as_ref().unwrap();
//     let head = repo.head().into_diagnostic()?;
//     let commit_id = head.peel_to_commit().into_diagnostic()?.id().to_string();
//     Ok(commit_id)
// }
