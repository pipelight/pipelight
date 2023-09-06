#[cfg(test)]
mod type_conversion {
    mod triggers {
        use crate::{Trigger, TriggerBranch, TriggerTag};

        #[test]
        fn normal() {
            let json = r#"
        {
            "branches": ["master"],
            "actions": ["pre-push"]
        }
        "#;
            let trigger = Trigger::TriggerBranch(TriggerBranch {
                branches: Some(vec!["master".to_owned()]),
                actions: Some(vec!["pre-push".to_owned()]),
            });
            let res = serde_json::from_str::<Trigger>(&json).unwrap();
            assert_eq!(trigger == res, true);
        }
        #[test]
        fn action_only() {
            let json = r#"
        {
            "actions": ["pre-push"]
        }
        "#;
            let trigger = Trigger::TriggerBranch(TriggerBranch {
                actions: Some(vec!["pre-push".to_owned()]),
                branches: None,
            });
            let res = serde_json::from_str::<Trigger>(&json).unwrap();
            assert_eq!(trigger == res, true);
        }
        #[test]
        fn action_only_wrong_enum() {
            let json = r#"
        {
            "actions": ["pre-push"]
        }
        "#;
            let trigger = Trigger::TriggerTag(TriggerTag {
                actions: Some(vec!["pre-push".to_owned()]),
                tags: None,
            });
            let res = serde_json::from_str::<Trigger>(&json).unwrap();
            assert_eq!(trigger == res, false);
        }
        #[test]
        fn array() {
            let json = r#"
        [
          {
              "branches": ["master"]
          },
          {
              "actions": ["manual", "watch"]
          }
        ]
        "#;
            let triggers = vec![
                Trigger::TriggerBranch(TriggerBranch {
                    branches: Some(vec!["master".to_owned()]),
                    actions: None,
                }),
                Trigger::TriggerBranch(TriggerBranch {
                    branches: None,
                    actions: Some(vec!["manual".to_owned(), "watch".to_owned()]),
                }),
            ];
            let res = serde_json::from_str::<Vec<Trigger>>(&json).unwrap();
            assert_eq!(triggers == res, true);
        }
    }
}
