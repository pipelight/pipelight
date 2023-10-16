#[cfg(test)]
mod config {
    use crate::types::{Config, Pipeline};
    use crate::{Trigger, TriggerBranch, TriggerTag};
    use utils::git::{Flag, Special};
    // Error Handling
    use miette::Result;

    #[test]
    fn has_any_watch_flag() {
        let config = Config {
            pipelines: Some(vec![Pipeline {
                triggers: Some(vec![Trigger::TriggerBranch(TriggerBranch {
                    action: Some(Flag::Special(Special::Watch)),
                    ..TriggerBranch::default()
                })]),
                ..Pipeline::default()
            }]),
        };
        let boolean = config.has_watchable().unwrap();
        assert!(boolean);
    }

    #[test]
    fn has_no_watch_flag() {
        let config = Config {
            pipelines: Some(vec![Pipeline {
                triggers: Some(vec![Trigger::TriggerBranch(TriggerBranch {
                    action: Some(Flag::Special(Special::Manual)),
                    ..TriggerBranch::default()
                })]),
                ..Pipeline::default()
            }]),
        };
        let boolean = config.has_watchable().unwrap();
        assert!(!boolean);
    }
}
