use git2::Repository;
use std::env;
use std::error::Error;

pub struct Git {
    pub path: String,
}
impl Git {
    pub fn new() -> Self {
        let mut e = Git {
            path: "./".to_owned(),
        };
        e.get().ok();
        return e;
    }
    ///  Detect if there is a git repo in pwd
    fn get(&mut self) -> Result<Repository, Box<dyn Error>> {
        // Seek git repo in current directory
        let root = env::current_dir()?;
        let repo = Repository::discover(root)?;
        self.path = repo.path().parent().unwrap().display().to_string();
        env::set_current_dir(&self.path)?;
        Ok(repo)
    }
}
