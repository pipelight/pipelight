// Traits
use serde::{Deserialize, Serialize};
// Trait - Enum iteration workaround
use strum::EnumIter;
// Git repository manipulation
use gix::Repository;

/**
Encapsulate the git repository struct to set it as optional
and add top level convenience methods for easier querying.
*/
pub struct Git {
    pub repo: Option<Repository>,
}

/**
An enumaration over the different types of flags(actions)
that can trigger a pipeline run.
*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Eq, Ord)]
#[serde(untagged)]
pub enum Flag {
    Hook(Hook),
    Special(Special),
}

/**
An enumaration over the different types of git-hooks(sub-flags)
that can trigger a pipeline run.
*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Eq, Ord, EnumIter)]
#[serde(rename_all = "kebab-case")]
pub enum Hook {
    // mail hooks
    ApplypatchMsg,
    PreApplypatch,
    PostApplypatch,
    SendemailValidate,
    // client hooks
    PreCommit,
    PreMergeCommit,
    PrepareCommitMsg,
    CommitMsg,
    PostCommit,
    // other client hooks
    PreRebase,
    PostCheckout,
    PostMerge,
    PrePush,
    PostRewrite,
    PreAutoGc,
    FsmonitorWatchman,
    PostIndexChange,
    // p4
    P4Changelist,
    P4PrepareChangelist,
    P4PostChangelist,
    P4PreSubmit,
    // server-side hooks
    PreReceive,
    Update,
    ProcReceive,
    PostReceive,
    PostUpdate,
    RefrenceTransaction,
    PushToCheckout,
}

/**
An enumaration over the different types of special flags(sub-flags)
that are external to git and involves other triggering methods
and that can trigger a pipeline run.
*/
#[derive(
    Default, Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Eq, Ord, EnumIter,
)]
#[serde(rename_all = "kebab-case")]
pub enum Special {
    #[default]
    Manual,
    Watch,
    Blank,
}
