use crate::git::{Flag, Special};

impl Default for Flag {
    fn default() -> Self {
        Flag::Special(Special::default())
    }
}
