#[cfg(test)]
mod display {
    use crate::error::{make_handler, CastError, JsonError};
    use crate::Pipeline;
    // use error::types::PipeError;
    use miette::{
        diagnostic, miette, Diagnostic, Error, IntoDiagnostic, MietteHandlerOpts, NamedSource,
        Result, RgbColors, SourceSpan,
    };
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
        make_handler()?;
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
        match res {
            Ok(_) => Ok(()),
            Err(err) => {
                let err = JsonError::new(err, json);
                Err(err.into())
            }
        }
    }
}
