#[cfg(test)]
mod try_teleport {
    use crate::teleport::{Gate, Portal};

    // Error Handling
    use miette::{IntoDiagnostic, Result};

    #[test]
    /// Search a pipelight.<extension> file in the filesystem
    fn _search() -> Result<()> {
        let mut portal = Portal::new()?;
        portal.seed("pipelight");
        let res = portal.search();
        assert!(res.is_ok());
        Ok(())
    }

    fn _search_unknow() -> Result<()> {
        let mut portal = Portal::new()?;
        portal.seed("test.pipelight.unknown");
        let res = portal.search();
        assert!(res.is_err());
        Ok(())
    }

    #[test]
    /// Search a config file with provided name in the filesystem
    fn _search_prefix() -> Result<()> {
        let internal = Gate {
            directory_path: Some("/home/areskul/Tools/PIPE/pipelight".to_owned()),
            file_path: Some("/home/areskul/Tools/PIPE/pipelight/test.pipelight.ts".to_owned()),
        };
        let mut portal = Portal::new()?;
        portal.seed("test.pipelight");
        portal.search_prefix()?;
        assert_eq!(internal, portal.target);
        Ok(())
    }
    #[test]
    /// Search a config file with provided name in the filesystem
    fn _search_prefix_unknown() -> Result<()> {
        let mut portal = Portal::new()?;
        portal.seed("test.pipelight.unknown");
        let res = portal.search_prefix();
        assert!(res.is_err());
        Ok(())
    }

    #[test]
    /// Search a config file with provided name in the filesystem
    fn _search_file() -> Result<()> {
        let name = "test.pipelight.ts";
        let internal = Gate {
            directory_path: Some("/home/areskul/Tools/PIPE/pipelight".to_owned()),
            file_path: Some("/home/areskul/Tools/PIPE/pipelight/test.pipelight.ts".to_owned()),
        };
        let mut portal = Portal::new()?.seed(name);
        portal.search_file()?;
        assert_eq!(internal, portal.target);
        Ok(())
    }

    #[test]
    /// Search a config file with provided name in the filesystem
    fn _search_file_unknown() -> Result<()> {
        let name = "test.pipelight.unknown.ts.";
        let res = Portal::new()?.seed(name).search_file();
        assert!(res.is_err());
        Ok(())
    }

    #[test]
    /// Search a config file with provided name in the filesystem
    fn _search_path() -> Result<()> {
        let path = "../test.pipelight.ts";
        let internal = Gate {
            directory_path: Some("/home/areskul/Tools/PIPE/pipelight".to_owned()),
            file_path: Some("/home/areskul/Tools/PIPE/pipelight/test.pipelight.ts".to_owned()),
        };
        let mut portal = Portal::new()?.seed(path);
        portal.search_path()?;
        assert_eq!(internal, portal.target);
        Ok(())
    }
    #[test]
    /// Search a config file with provided name in the filesystem
    fn _search_path_unknown() -> Result<()> {
        let path = "./test.pipelight.unknown.ts";
        let res = Portal::new()?.seed(path).search_path();
        assert!(res.is_err());
        Ok(())
    }
}
