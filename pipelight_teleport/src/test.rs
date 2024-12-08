#[cfg(test)]
mod try_teleport {
    use crate::{Gate, Portal};
    use std::env;

    // Error Handling
    use miette::Result;

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

    // #[test]
    /// Search a config file with provided name in the filesystem
    fn _search_prefix() -> Result<()> {
        // Test pwd
        let pwd = env::current_dir().unwrap();
        let pwd = pwd.to_str().unwrap().to_owned();
        let internal = Gate {
            directory_path: Some(pwd.clone()),
            file_path: Some(pwd + "/test.pipelight.ts"),
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

    // #[test]
    /// Search a config file with provided name in the filesystem
    fn _search_file() -> Result<()> {
        let name = "test.pipelight.ts";

        let pwd = env::current_dir().unwrap();
        let pwd = pwd.to_str().unwrap().to_owned();
        let internal = Gate {
            directory_path: Some(pwd.clone()),
            file_path: Some(pwd + "/test.pipelight.ts"),
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

    // #[test]
    /// Search a config file with provided name in the filesystem
    fn _search_path() -> Result<()> {
        let path = "../test.pipelight.ts";
        let pwd = env::current_dir().unwrap();
        let pwd = pwd.to_str().unwrap().to_owned();
        let internal = Gate {
            directory_path: Some(pwd.clone()),
            file_path: Some(pwd + "/test.pipelight.ts"),
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
