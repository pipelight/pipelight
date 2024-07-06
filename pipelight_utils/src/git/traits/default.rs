// Git
use git2::Repository;
// Environment manipulation
use std::env;
// Structs
use crate::git::{Flag, Git, Special};

impl Git {
    pub fn new() -> Self {
        Self::default()
    }
}
impl Default for Git {
    fn default() -> Git {
        let root = env::current_dir().unwrap();
        Git {
            // recursively browse through fs
            repo: Repository::discover(root).ok(),
        }
    }
}

impl Default for Flag {
    fn default() -> Self {
        Flag::Special(Special::default())
    }
}
