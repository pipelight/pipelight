// Filesystem
use std::path::Path;
// Error Handling
use miette::{Error, Result};

/**
The following boolean or each mutualy exclusives.
A path cannot be more than one of the three alternatives.
It is either a filename, a relative path or an absolute path
*/

/**
Determine whether the provided path struct is a filename
*/
pub fn is_filename(path: &Path) -> Result<()> {
    if path == Path::new(path.file_name().unwrap()) {
        Ok(())
    } else {
        Err(Error::msg("The provided path isn't a filename."))
    }
}
/**
Determine whether the provided path struct is a relative path
Return false if the path is a filename like "my_file" and isn't prefixed with "./".
Return true if the file path is "./my_file".

This is a tweaked strict version of the standard Path::is_relative() method.
*/
pub fn is_relative(path: &Path) -> Result<()> {
    if path.is_relative() && self::is_filename(path).is_err() {
        Ok(())
    } else {
        Err(Error::msg("The provided path isn't a relative path."))
    }
}
/**
Determine whether the provided path struct is an absolute path
*/
pub fn is_absolute(path: &Path) -> Result<()> {
    if path.is_absolute() {
        Ok(())
    } else {
        Err(Error::msg("The provided path isn't an absolute path."))
    }
}
