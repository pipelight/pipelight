use git2::{Reference, Repository};
use std::env;
use std::error::Error;

pub struct Git {
    pub repo: Option<Repository>,
}
impl Git {
    pub fn new() -> Self {
        let mut e = Git { repo: None };
        &e.get();
        return e;
    }
    ///  Detect if there is a git repo in pwd
    fn get(&mut self) -> Result<(), Box<dyn Error>> {
        // Seek git repo in current directory
        let root = env::current_dir()?;
        let repo = Repository::discover(root)?;
        // Set working dir
        let wd = repo.workdir().unwrap().display().to_string();
        env::set_current_dir(wd)?;

        self.repo = Some(repo);
        Ok(())
    }
    pub fn branch(self) -> Result<String, Box<dyn Error>> {
        // Only tested on attached HEAD
        // No edge case when head is a commit or else...
        let repo = self.repo.unwrap();
        let head = repo.head()?;
        let name = head.shorthand().unwrap().to_owned();
        Ok(name)
    }
}
