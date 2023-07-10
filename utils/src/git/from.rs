use super::Hook::*;
use super::{Flag, Hook};
use convert_case::{Case, Casing};
use log::error;
use std::fmt;
use std::process::exit;

impl fmt::Display for Hook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<&String> for Hook {
    fn from(action: &String) -> Hook {
        // let cased: &str = &action.to_case(Case::Snake);
        let cased: &str = &action.to_case(Case::Kebab);
        match cased {
            // client hooks
            "pre-commit" => PreCommit,
            "prepare-commit-msg" => PrepareCommitMsg,
            "commit-msg" => CommitMsg,
            "post-commit" => PostCommit,
            // mail hooks
            "applypatch-msg" => ApplypatchMsg,
            "pre-applypatch" => PreApplypatch,
            "post-applypatch" => PostApplypatch,
            // other client hooks
            "pre-rebase" => PreRebase,
            "post-rewrite" => PostRewrite,
            "post-checkout" => PostCheckout,
            "post-merge" => PostMerge,
            "pre-push" => PrePush,
            "pre-auto-gc" => PreAutoGc,
            // server-side hooks
            "pre-receive" => PreReceive,
            "update" => Update,
            "post-update" => PostUpdate,
            "post-receive" => PostReceive,
            _ => {
                let message = format!("The hook {} is not known", cased);
                error!("{}", message);
                exit(1);
            }
        }
    }
}
impl From<&Hook> for String {
    fn from(action: &Hook) -> String {
        match action {
            ApplypatchMsg => "applypatch-msg".to_owned(),
            PreApplypatch => "pre-apply-patch".to_owned(),
            PostApplypatch => "post-apply-patch".to_owned(),
            PreCommit => "pre-commit".to_owned(),
            PrepareCommitMsg => "prepare-commit-msg".to_owned(),
            CommitMsg => "commit-msg".to_owned(),
            PostCommit => "post-commit".to_owned(),
            PreRebase => "pre-rebase".to_owned(),
            PostCheckout => "post-checkout".to_owned(),
            PostMerge => "post-merge".to_owned(),
            PreReceive => "pre-receive".to_owned(),
            Update => "update".to_owned(),
            PostReceive => "post-receive".to_owned(),
            PostUpdate => "post-update".to_owned(),
            PreAutoGc => "pre-auto-gc".to_owned(),
            PostRewrite => "post-rewrite".to_owned(),
            PrePush => "pre-push".to_owned(),
        }
    }
}
impl From<&String> for Flag {
    fn from(action: &String) -> Flag {
        let cased: &str = &action.to_case(Case::Kebab);
        match cased {
            "manual" => Flag::Manual,
            "watch" => Flag::Watch,
            _ => Flag::Hook(Hook::from(action)),
        }
    }
}
impl From<&Flag> for String {
    fn from(action: &Flag) -> String {
        match action {
            Flag::Manual => "manual".to_owned(),
            Flag::Watch => "watch".to_owned(),
            Flag::Hook(hook) => String::from(hook),
        }
    }
}
