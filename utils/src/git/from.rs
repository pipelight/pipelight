use super::Hook;
use convert_case::{Case, Casing};
use std::fmt;

impl fmt::Display for Hook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
