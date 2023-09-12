#[cfg(test)]
mod serialize {
    use crate::workflow::Trigger;
    use utils::git::{Flag, Hook, Special};

    #[test]
    fn try_serialize_trigger_hook() {
        let env = Trigger {
            action: Some(Flag::Hook(Hook::PrePush)),
            branch: Some("master".to_owned()),
            tag: None,
        };
        let res = serde_json::to_string::<Trigger>(&env).unwrap();

        let mut json = r#"
        {
            "action": "pre-push",
            "branch": "master",
            "tag": null
        }
        "#;
        let binding = json.replace(" ", "").replace("\n", "");
        json = &binding;
        assert_eq!(res, json);
    }
    #[test]
    fn try_serialize_trigger_special() {
        let env = Trigger {
            action: Some(Flag::Special(Special::Manual)),
            branch: Some("master".to_owned()),
            tag: None,
        };
        let res = serde_json::to_string::<Trigger>(&env).unwrap();

        let mut json = r#"
        {
            "action": "manual",
            "branch": "master",
            "tag": null
        }
        "#;
        let binding = json.replace(" ", "").replace("\n", "");
        json = &binding;
        assert_eq!(res, json);
    }
    #[test]
    fn try_deserialize_trigger_hook() {
        let env = Trigger {
            action: Some(Flag::Hook(Hook::PrePush)),
            branch: Some("master".to_owned()),
            tag: None,
        };
        let json = r#"
        {
            "action": "pre-push",
            "branch": "master",
            "tag": null
        }
        "#;
        let res = serde_json::from_str::<Trigger>(&json).unwrap();
        assert_eq!(res, env);
    }
}
