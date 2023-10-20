#[cfg(test)]
mod trigger_match_no_git {
    use crate::{Trigger, TriggerBranch, TriggerTag};
    use utils::git::{Flag, Hook, Special};

    #[test]
    fn try_match_flag() {
        let env = Trigger::TriggerBranch(TriggerBranch {
            branch: None,
            action: Some(Flag::Special(Special::Manual)),
        });
        let triggers = vec![Trigger::TriggerBranch(TriggerBranch {
            branch: None,
            action: Some(Flag::Special(Special::Manual)),
        })];
        assert_eq!(env.has_match(triggers).unwrap(), true);
    }
}
#[cfg(test)]
mod trigger_match_git {
    use crate::{Trigger, TriggerBranch, TriggerTag};
    use utils::git::{Flag, Hook, Special};

    /// match trigger with branch without action
    #[test]
    fn try_match_branch() {
        let env = Trigger::TriggerBranch(TriggerBranch {
            branch: Some("master".to_owned()),
            action: Some(Flag::Hook(Hook::PrePush)),
        });
        let triggers = vec![Trigger::TriggerBranch(TriggerBranch {
            branch: Some("master".to_owned()),
            action: None,
        })];
        assert_eq!(env.has_match(triggers).unwrap(), true);
    }
    /// unmatch trigger with branch without action
    #[test]
    fn try_unmatch_branch() {
        let env = Trigger::TriggerBranch(TriggerBranch {
            branch: Some("master".to_owned()),
            action: Some(Flag::Hook(Hook::PrePush)),
        });
        let triggers = vec![Trigger::TriggerBranch(TriggerBranch {
            branch: Some("dev".to_owned()),
            action: None,
        })];
        assert_eq!(env.has_match(triggers).unwrap(), false);
    }
    /// match trigger with action without branch
    #[test]
    fn try_match_action() {
        let env = Trigger::TriggerBranch(TriggerBranch {
            branch: Some("master".to_owned()),
            action: Some(Flag::Hook(Hook::PrePush)),
        });
        let triggers = vec![Trigger::TriggerBranch(TriggerBranch {
            branch: None,
            action: Some(Flag::Hook(Hook::PrePush)),
        })];
        assert_eq!(env.has_match(triggers).unwrap(), true);
    }
    #[test]
    /// unmatch trigger with action without branch
    fn try_unmatch_action() {
        let env = Trigger::TriggerBranch(TriggerBranch {
            branch: Some("master".to_owned()),
            action: Some(Flag::Special(Special::Watch)),
        });
        let triggers = vec![Trigger::TriggerBranch(TriggerBranch {
            branch: None,
            action: Some(Flag::Hook(Hook::PrePush)),
        })];
        assert_eq!(env.has_match(triggers).unwrap(), false);
    }
    #[test]
    /// match trigger with tag without action
    fn try_match_tag() {
        let env = Trigger::TriggerTag(TriggerTag {
            tag: Some("v0.4".to_owned()),
            action: Some(Flag::Hook(Hook::PrePush)),
        });
        let triggers = vec![Trigger::TriggerTag(TriggerTag {
            tag: Some("v0.4".to_owned()),
            action: None,
        })];
        assert_eq!(env.has_match(triggers).unwrap(), true);
    }
    #[test]
    /// unmatch trigger with tag without action
    fn try_unmatch_tag() {
        let env = Trigger::TriggerTag(TriggerTag {
            tag: Some("v0.5".to_owned()),
            action: Some(Flag::Hook(Hook::PrePush)),
        });
        let triggers = vec![Trigger::TriggerTag(TriggerTag {
            tag: Some("v0.4".to_owned()),
            action: None,
        })];
        assert_eq!(env.has_match(triggers).unwrap(), false);
    }
    #[test]
    fn try_match_none() {
        let env = Trigger::TriggerTag(TriggerTag {
            tag: Some("v0.5".to_owned()),
            action: Some(Flag::Hook(Hook::PrePush)),
        });
        let triggers = vec![Trigger::TriggerTag(TriggerTag {
            tag: None,
            action: None,
        })];
        assert_eq!(env.has_match(triggers).unwrap(), true);
    }
    #[test]
    fn try_match_action_no_tag() {
        let env = Trigger::TriggerTag(TriggerTag {
            tag: Some("v0.5".to_owned()),
            action: Some(Flag::Hook(Hook::PrePush)),
        });
        let triggers = vec![Trigger::TriggerTag(TriggerTag {
            tag: None,
            action: Some(Flag::Hook(Hook::PrePush)),
        })];
        assert_eq!(env.has_match(triggers).unwrap(), true);
    }
    #[test]
    fn try_unmatch_action_no_tag() {
        let env = Trigger::TriggerTag(TriggerTag {
            tag: Some("v0.5".to_owned()),
            action: Some(Flag::Hook(Hook::PrePush)),
        });
        let triggers = vec![Trigger::TriggerTag(TriggerTag {
            tag: None,
            action: Some(Flag::Hook(Hook::PreCommit)),
        })];
        assert_eq!(env.has_match(triggers).unwrap(), false);
    }
}
