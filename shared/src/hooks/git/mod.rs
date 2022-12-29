use git2::Repository;
use std::env;
use std::error::Error;

pub struct Git {
    path: String,
}
impl Git {
    pub fn new(&mut self) -> Result<Repository, Box<dyn Error>> {
        Ok(self.get()?)
    }
    ///  Detect if there is a git repo in pwd
    pub fn get(&mut self) -> Result<Repository, Box<dyn Error>> {
        match Git::get_from_wd() {
            Ok(res) => {
                self.path = res.path().display().to_string();
                return Ok(res);
            }
            Err(e) => match Git::get_from_hook() {
                Ok(res) => {
                    self.path = res.path().display().to_string();
                    return Ok(res);
                }
                Err(e) => return Err(e),
            },
        }
    }
    fn get_from_wd() -> Result<Repository, Box<dyn Error>> {
        // Seek git repo in current directory
        let root = env::current_dir()?;
        let repo = Repository::discover(root)?;
        Ok(repo)
    }
    fn get_from_hook() -> Result<Repository, Box<dyn Error>> {
        // Seek git repo from inside git hook folder
        let hook_dir = env::current_dir()?;
        let root = hook_dir
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap();
        let repo = Repository::discover(root)?;
        Ok(repo)
    }
}
