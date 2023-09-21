use std::path::Path;

// Error Handling
use miette::{Error, IntoDiagnostic, Result};

pub fn is_filename(path: &Path) -> Result<()> {
    if path == Path::new(path.file_name().unwrap()) {
        Ok(())
    } else {
        Err(Error::msg("The provided path isn't a filename."))
    }
}
pub fn is_relative(path: &Path) -> Result<()> {
    if path.is_relative() && self::is_filename(path).is_err() {
        Ok(())
    } else {
        Err(Error::msg("The provided path isn't a relative path."))
    }
}
pub fn is_absolute(path: &Path) -> Result<()> {
    if path.is_absolute() {
        Ok(())
    } else {
        Err(Error::msg("The provided path isn't an absolute path."))
    }
}
