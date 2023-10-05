#[cfg(test)]
mod config {
    use crate::types::{Config, Pipeline, Trigger};
    use utils::git::{Flag, Special};
    // Error Handling
    use miette::Result;

    #[test]
    fn has_watch_any_flag() {
        let config = Config {
            pipelines: Some(vec![Pipeline {
                triggers: Some(vec![Trigger {
                    action: Some(Flag::Special(Special::Watch)),
                    ..Trigger::default()
                }]),
                ..Pipeline::default()
            }]),
        };
        let boolean = config.has_watch_flag().unwrap();
        assert!(boolean);
    }

    #[test]
    fn has_watch_no_flag() {
        let config = Config {
            pipelines: Some(vec![Pipeline {
                triggers: Some(vec![Trigger {
                    action: Some(Flag::Special(Special::Manual)),
                    ..Trigger::default()
                }]),
                ..Pipeline::default()
            }]),
        };
        let boolean = config.has_watch_flag().unwrap();
        assert!(!boolean);
    }
}
