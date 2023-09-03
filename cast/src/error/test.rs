#[cfg(test)]
mod display {
    use crate::Pipeline;
    use miette::Result;
    #[test]
    fn wrong_json_type() -> Result<()> {
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
        let res = serde_json::from_str::<Pipeline>(&json);
        assert!(res.is_err());
        Ok(())
    }
    #[test]
    fn pipeline_bad_name() -> Result<()> {
        let json = r#"
          {
            "name": "my pipe",
            "steps":[
              {
                "name": "my step",
                "command": ["test"]
              }
            ]
          }
        "#;
        let res = serde_json::from_str::<Pipeline>(&json);
        assert!(res.is_err());
        Ok(())
    }
    #[test]
    fn wrong_yaml() -> Result<()> {
        let yml = r#"
        name: test
        steps:
          - command:
              - ls
            name: list directory
          - commands:
              - pwd
            name: get working directory
        "#;
        let res = serde_yaml::from_str::<Pipeline>(&yml);
        assert!(res.is_err());
        Ok(())
    }
}
