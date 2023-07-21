#[cfg(test)]
mod serialize {
    use crate::Trigger;
    use utils::git::{Flag, Hook};
    // Error Handling
    use miette::{IntoDiagnostic, Result};

    #[test]
    /// match trigger with branch without action
    fn try_serialize_trigger() {
        let env = Trigger {
            action: Some(Flag::Hook(Hook::PrePush)),
            branch: Some("master".to_owned()),
            tag: None,
        };
        let res = serde_json::to_string_pretty::<Trigger>(&env).unwrap();

        let json = r#"
        {
            "action": "pre-push"
            "branch": "master",
            "tag": null,
        }
        "#;
        println!("{:?}", env);
        println!("{:?}", res);
        assert_eq!(res, json);
    }
    fn try_serialize_flag() {
        let env = Trigger {
            action: Some(Flag::Hook(Hook::PrePush)),
            branch: Some("master".to_owned()),
            tag: None,
        };
        let res = serde_json::to_string_pretty::<Trigger>(&env).unwrap();

        let json = r#"
        {
            "action": "pre-push"
            "branch": "master",
            "tag": null,
        }
        "#;
        println!("{:?}", env);
        println!("{:?}", res);
        assert_eq!(res, json);
    }
}
