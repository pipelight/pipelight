#[cfg(test)]
mod try_teleport {
    use crate::teleport::{Internal, NaiveFileInfo, Teleport};

    // Error Handling
    use miette::{IntoDiagnostic, Result};

    #[test]
    /// Search a pipelight.<extension> file in the filesystem
    fn _search() -> Result<()> {
        let mut teleport = Teleport::new();
        let res = teleport.search_preffix();
        assert!(res.is_ok());
        Ok(())
    }

    fn _search_unknow() -> Result<()> {
        let mut teleport = Teleport {
            file_info: NaiveFileInfo {
                preffix: "test.pipelight.unknown".to_owned(),
                ..NaiveFileInfo::default()
            },
            ..Teleport::default()
        };
        let res = teleport.search_preffix();
        assert!(res.is_err());
        Ok(())
    }

    #[test]
    /// Search a config file with provided name in the filesystem
    fn _search_preffix() -> Result<()> {
        let internal = Internal {
            directory_path: Some("/home/areskul/Tools/PIPE/pipelight".to_owned()),
            file_path: Some("/home/areskul/Tools/PIPE/pipelight/test.pipelight.ts".to_owned()),
        };
        let mut teleport = Teleport {
            file_info: NaiveFileInfo {
                preffix: "test.pipelight".to_owned(),
                ..NaiveFileInfo::default()
            },
            ..Teleport::default()
        };
        teleport.search_preffix()?;
        assert_eq!(internal, teleport.internal);
        Ok(())
    }
    #[test]
    /// Search a config file with provided name in the filesystem
    fn _search_preffix_unknown() -> Result<()> {
        let mut teleport = Teleport {
            file_info: NaiveFileInfo {
                preffix: "test.pipelight.unknown".to_owned(),
                ..NaiveFileInfo::default()
            },
            ..Teleport::default()
        };
        let res = teleport.search_preffix();
        assert!(res.is_err());
        Ok(())
    }

    #[test]
    /// Search a config file with provided name in the filesystem
    fn _search_file() -> Result<()> {
        let name = "test.pipelight.ts";
        let internal = Internal {
            directory_path: Some("/home/areskul/Tools/PIPE/pipelight".to_owned()),
            file_path: Some("/home/areskul/Tools/PIPE/pipelight/test.pipelight.ts".to_owned()),
        };
        let mut teleport = Teleport::new().file(name)?;
        teleport.search_file()?;
        assert_eq!(internal, teleport.internal);
        Ok(())
    }

    #[test]
    /// Search a config file with provided name in the filesystem
    fn _search_file_unknown() -> Result<()> {
        let name = "test.pipelight.unknown.ts.";
        let res = Teleport::new().file(name)?.search_file();
        assert!(res.is_err());
        Ok(())
    }

    #[test]
    /// Search a config file with provided name in the filesystem
    fn _search_path() -> Result<()> {
        let path = "../test.pipelight.ts";
        let internal = Internal {
            directory_path: Some("/home/areskul/Tools/PIPE/pipelight".to_owned()),
            file_path: Some("/home/areskul/Tools/PIPE/pipelight/test.pipelight.ts".to_owned()),
        };
        let mut teleport = Teleport::new().file(path)?;
        teleport.search_path()?;
        assert_eq!(internal, teleport.internal);
        Ok(())
    }
    #[test]
    /// Search a config file with provided name in the filesystem
    fn _search_path_unknown() -> Result<()> {
        let path = "./test.pipelight.unknown.ts";
        let res = Teleport::new().file(path)?.search_path();
        assert!(res.is_err());
        Ok(())
    }
}
