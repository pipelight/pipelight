#[cfg(test)]
mod test_actions {
    // Struct
    use crate::types::Action;
    // Error Handling
    use miette::{IntoDiagnostic, Result};

    #[test]
    fn create_action() -> Result<()> {
        Action::Run(Some("test".to_owned())).start()?;
        Ok(())
    }
}
