#[cfg(test)]
mod tests {
    use crate::src::types::*;
    use log::LevelFilter;
    use utils::logger::logger;

    #[test]
    /// Loads a js config with a simple pipeline
    /// Required fields only
    fn load_config_required() {
        let js = r#"
        const config = {
          pipelines: [
            {
              name: "default",
              steps: [
                {
                  name: "wait",
                  commands: ["ls", "sleep 15", "pwd"],
                },
              ],
            }
        }"#;
    }
    #[test]
    /// Loads a js config with a simple pipeline
    /// Missing required fields
    fn load_config_missing_required() {
        let js = r##"
        const config = {
          pipelines: [
            {
              // name: "default",
              steps: [
                {
                  // name: "wait",
                  // commands: ["ls", "sleep 15", "pwd"],
                },
              ],
            }
        }"##;
    }
    #[test]
    /// Loads a js config with parallel steps
    fn load_config_parallel() {
        let js = r##"
        const config = {
          pipelines: [
            {
              // name: "default_para",
              steps: [{
                parallel:[
                    {
                      name: "wait",
                      commands: ["ls", "sleep 15", "pwd"],
                    },
                    {
                      name: "wait2",
                      commands: ["ls", "sleep 15", "pwd"],
                    },
                ]
                }],
            }]
        }"##;
    }
}
