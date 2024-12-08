#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Portal {
    pub seed: Option<String>,
    /// Process origin path
    pub origin: Gate,
    /// Process current path
    pub current: Gate,
    pub target: Gate,
}
#[derive(Default, Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
/// Internal values to browse the fs
pub struct Gate {
    /// Seek file dir path
    pub directory_path: Option<String>,
    /// Seek file file_path.
    pub file_path: Option<String>,
}
