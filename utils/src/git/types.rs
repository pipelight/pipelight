// External Import
use git2::Repository;
use serde::{Deserialize, Serialize};
// Enum workaround
use std::string::ToString;
use strum::{EnumIter, IntoEnumIterator};

pub struct Git {
    pub repo: Option<Repository>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Eq, Ord)]
#[serde(untagged)]
pub enum Flag {
    Hook(Hook),
    Special(Special),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd, EnumIter, Eq, Ord)]
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

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Eq, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum Special {
    #[default]
    Manual,
    Watch,
    Blank,
}
