use super::Hook;
use convert_case::{Case, Casing};
use std::fmt;

// impl From<&str> for Hook {
//     /// Convert str into enum GitHook
//     fn from(name: &str) -> Self {
//         let hook = &name.to_case(Case::Pascal);
//         return hook;
//     }
// }
impl fmt::Display for Hook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
