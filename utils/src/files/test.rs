#[cfg(test)]
mod is {
    use crate::files::is;
    use std::path::Path;

    #[test]
    fn try_is_filename() {
        let path = Path::new("test.txt");
        let res = is::is_filename(&path);
        assert!(res.is_ok());
    }
    #[test]
    fn try_is_not_filename_but_relative() {
        let path = Path::new("directory/test.txt");
        let res = is::is_filename(&path);
        assert!(res.is_err());
    }
    #[test]
    fn try_is_not_filename_but_absolute() {
        let path = Path::new("/directory/test.txt");
        let res = is::is_filename(&path);
        assert!(res.is_err());
    }
    #[test]
    fn try_is_relative() {
        let path = Path::new("directory/test.txt");
        let res = is::is_relative(&path);
        assert!(res.is_ok());
    }
    #[test]
    fn try_is_not_relative_but_filename() {
        let path = Path::new("test.txt");
        let res = is::is_relative(&path);
        assert!(res.is_err());
    }
    #[test]
    fn try_is_not_relative_but_absolute() {
        let path = Path::new("/directory/test.txt");
        let res = is::is_relative(&path);
        assert!(res.is_err());
    }
    #[test]
    fn try_is_absolute() {
        let path = Path::new("/directory/test.txt");
        let res = is::is_absolute(&path);
        assert!(res.is_ok());
    }
    #[test]
    fn try_is_not_absolute_but_relative() {
        let path = Path::new("directory/test.txt");
        let res = is::is_absolute(&path);
        assert!(res.is_err());
    }
    #[test]
    fn try_is_not_absolute_but_filename() {
        let path = Path::new("test.txt");
        let res = is::is_absolute(&path);
        assert!(res.is_err());
    }
}
