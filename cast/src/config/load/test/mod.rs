// Serde conversion test
mod conversion;

#[cfg(test)]
mod cast {
    use crate::{Config, Logs};
    #[test]
    fn toml() {
        let res = Config::load("./public/pipelight.toml", None);
        assert!(res.is_ok());
    }
    #[test]
    fn yaml() {
        let res = Config::load("./public/pipelight.yaml", None);
        assert!(res.is_ok());
    }
    #[test]
    fn javascript() {
        let res = Config::load("./public/pipelight.js", None);
        assert!(res.is_ok());
    }
    #[test]
    fn typescript() {
        let res = Config::load("./public/pipelight.ts", None);
        assert!(res.is_ok());
    }
    #[test]
    fn logs() {
        let res = Logs::read("./public");
        assert!(res.is_ok());
    }
}
