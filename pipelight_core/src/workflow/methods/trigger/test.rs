#[cfg(test)]
mod trigger_match {
    use crate::workflow::Trigger;
    use utils::git::{Flag, Hook, Special};

    #[test]
    /// match trigger with branch without action
    fn try_match_branch() {
        let env = Trigger {
            branch: Some("master".to_owned()),
            action: Some(Flag::Hook(Hook::PrePush)),
            tag: None,
        };
        let triggers = vec![Trigger {
            branch: Some("master".to_owned()),
            action: None,
            tag: None,
        }];
        assert!(env.is_match(triggers).is_ok());
    }
    #[test]
    /// unmatch trigger with branch without action
    fn try_unmatch_branch() {
        let env = Trigger {
            branch: Some("master".to_owned()),
            action: Some(Flag::Hook(Hook::PrePush)),
            tag: None,
        };
        let triggers = vec![Trigger {
            branch: Some("dev".to_owned()),
            action: None,
            tag: None,
        }];
        assert!(env.is_match(triggers).is_err());
    }
    #[test]
    /// match trigger with action without branch
    fn try_match_action() {
        let env = Trigger {
            branch: Some("master".to_owned()),
            action: Some(Flag::Hook(Hook::PrePush)),
            tag: None,
        };
        let triggers = vec![Trigger {
            branch: None,
            action: Some(Flag::Hook(Hook::PrePush)),
            tag: None,
        }];
        assert!(env.is_match(triggers).is_ok());
    }
    #[test]
    /// unmatch trigger with action without branch
    fn try_unmatch_action() {
        let env = Trigger {
            branch: Some("master".to_owned()),
            action: Some(Flag::Special(Special::Watch)),
            tag: None,
        };
        let triggers = vec![Trigger {
            branch: None,
            action: Some(Flag::Hook(Hook::PrePush)),
            tag: None,
        }];
        assert!(env.is_match(triggers).is_err());
    }
    #[test]
    /// match trigger with tag without action
    fn try_match_tag() {
        let env = Trigger {
            tag: Some("v0.4".to_owned()),
            action: Some(Flag::Hook(Hook::PrePush)),
            branch: None,
        };
        let triggers = vec![Trigger {
            tag: Some("v0.4".to_owned()),
            action: None,

            branch: None,
        }];
        assert!(env.is_match(triggers).is_ok());
    }
    #[test]
    /// unmatch trigger with tag without action
    fn try_unmatch_tag() {
        let env = Trigger {
            tag: Some("v0.5".to_owned()),
            action: Some(Flag::Hook(Hook::PrePush)),
            branch: None,
        };
        let triggers = vec![Trigger {
            tag: Some("v0.4".to_owned()),
            action: None,
            branch: None,
        }];
        assert!(env.is_match(triggers).is_err());
    }
    #[test]
    /// match trigger enums
    fn try_match_enum() {
        let env = Trigger {
            tag: Some("v0.5".to_owned()),
            action: Some(Flag::Hook(Hook::PrePush)),
            branch: None,
        };
        let triggers = vec![Trigger {
            branch: None,
            action: None,
            tag: None,
        }];
        assert!(env.is_match(triggers).is_ok());
    }
    #[test]
    fn try_match_action_no_tag() {
        let env = Trigger {
            tag: Some("v0.5".to_owned()),
            action: Some(Flag::Hook(Hook::PrePush)),
            branch: None,
        };
        let triggers = vec![Trigger {
            tag: None,
            action: Some(Flag::Hook(Hook::PrePush)),
            branch: None,
        }];
        assert!(env.is_match(triggers).is_ok());
    }
    #[test]
    fn try_unmatch_action_no_tag() {
        let env = Trigger {
            tag: Some("v0.5".to_owned()),
            action: Some(Flag::Hook(Hook::PrePush)),
            branch: None,
        };
        let triggers = vec![Trigger {
            tag: None,
            action: Some(Flag::Hook(Hook::PreCommit)),
            branch: None,
        }];
        assert!(env.is_match(triggers).is_err());
    }
}
