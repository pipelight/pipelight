// Error Handling
use miette::{Diagnostic, Error, IntoDiagnostic, NamedSource, Report, Result, SourceSpan};
use thiserror::Error;

pub struct BoxedError(pub Box<dyn Error + Send + Sync>);
impl fmt::Debug for BoxedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}
impl fmt::Display for BoxedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}
impl Error for BoxedError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.0.source()
    }
    #[allow(deprecated)]
    fn description(&self) -> &str {
        self.0.description()
    }
    #[allow(deprecated)]
    fn cause(&self) -> Option<&dyn Error> {
        self.0.cause()
    }
}

#[derive(Debug, Error, Diagnostic)]
pub enum MyError<ErrType: 'static + Error = BoxedError> {
    #[error("Error containing another error")]
    AnError(#[source] ErrType),
}

#[derive(Error, Debug, Diagnostic)]
#[error("js file syntax issue!")]
#[diagnostic(code(js::error))]
struct JsError {
    #[source_code]
    src: NamedSource,
    #[label("This bit here")]
    bad_bit: SourceSpan,
}
