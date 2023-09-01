#[cfg(test)]
mod display {
    use crate::code_location;
    use crate::types::{make_handler, CliError, PipeError};
    use miette::{
        diagnostic, miette, Diagnostic, Error, IntoDiagnostic, NamedSource, Result, SourceSpan,
    };
    use std::fs::File;
    use thiserror::Error;
    #[test]
    fn display_code_span() -> Result<()> {
        make_handler()?;
        let res = File::open("foo.txt");

        println!("{:#?}", res);
        match res {
            Ok(_) => Ok(()),
            Err(err) => {
                let err = PipeError::default();
                Err(err.into())
            }
        }
        // println!("{:#?}", res);
        // println!("error from file: {}", file!());
        // println!("error from line: {}", line!());
        // Ok(())
    }
}
