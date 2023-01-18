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
            "applypatch-msg" => return ApplypatchMsg,
            "pre-apply-patch" => return PreApplypatch,
            "post-apply-patch" => return PostApplypatch,
            "pre-commit" => return PreCommit,
            "prepare-commit-msg" => return PrepareCommitMsg,
            "commit-msg" => return CommitMsg,
            "post-commit" => return PostCommit,
            "pre-rebase" => return PreRebase,
            "post-checkout" => return PostCheckout,
            "post-merge" => return PostMerge,
            "pre-receive" => return PreReceive,
            "update" => return Update,
            "post-receive" => return PostReceive,
            "post-update" => return PostUpdate,
            "pre-auto-gc" => return PreAutoGc,
            "post-rewrite" => return PostRewrite,
            "pre-push" => return PrePush,
            _ => {
                let message = format!("The hook {} is not known", cased);
                error!("{}", message);
                exit(1);
            }
        };
    }
}
impl From<&Hook> for String {
    fn from(action: &Hook) -> String {
        match action {
            ApplypatchMsg => return "applypatch-msg".to_owned(),
            PreApplypatch => return "pre-apply-patch".to_owned(),
            PostApplypatch => return "post-apply-patch".to_owned(),
            PreCommit => return "pre-commit".to_owned(),
            PrepareCommitMsg => return "prepare-commit-msg".to_owned(),
            CommitMsg => return "commit-msg".to_owned(),
            PostCommit => return "post-commit".to_owned(),
            PreRebase => return "pre-rebase".to_owned(),
            PostCheckout => return "post-checkout".to_owned(),
            PostMerge => return "post-merge".to_owned(),
            PreReceive => return "pre-receive".to_owned(),
            Update => return "update".to_owned(),
            PostReceive => return "post-receive".to_owned(),
            PostUpdate => return "post-update".to_owned(),
            PreAutoGc => return "pre-auto-gc".to_owned(),
            PostRewrite => return "post-rewrite".to_owned(),
            PrePush => return "pre-push".to_owned(),
        };
    }
}
impl From<&String> for Flag {
    fn from(action: &String) -> Flag {
        let cased: &str = &action.to_case(Case::Kebab);
        if cased == "manual" {
            return Flag::Manual;
        } else {
            return Flag::Hook(Hook::from(action));
        }
    }
}
impl From<&Flag> for String {
    fn from(action: &Flag) -> String {
        match action {
            Flag::Manual => return "manual".to_owned(),
            Flag::Hook(hook) => return String::from(hook),
        };
    }
}
