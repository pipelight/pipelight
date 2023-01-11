use super::Hook;
use super::Hook::*;
use convert_case::{Case, Casing};
use std::error::Error;
use std::fmt;

impl fmt::Display for Hook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<&str> for Hook {
    fn from(mut action: &str) -> Hook {
        match action {
            "applypatch-msg" => ApplypatchMsg,
            "pre-apply-patch" => PreApplypatch,
            "post-apply-patch" => PostApplypatch,
            "pre-commit" => PreCommit,
            "prepare-commit-msg" => PrepareCommitMsg,
            "commit-msg" => CommitMsg,
            "post-commit" => PostCommit,
            "pre-rebase" => PreRebase,
            "post-checkout" => PostCheckout,
            "post-merge" => PostMerge,
            "pre-receive" => PreReceive,
            "update" => Update,
            "post-receive" => PostReceive,
            "post-update" => PostUpdate,
            "pre-auto-gc" => PreAutoGc,
            "post-rewrite" => PostRewrite,
            "pre-push" => PrePush,
        };
    }
}
