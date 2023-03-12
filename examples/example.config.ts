import { Config } from "npm:pipelight";
const config: Config = {
  pipelines: [
    {
      name: "simple_example",
      steps: [
        {
          name: "list files",
          commands: ["ls"]
        },
        {
          name: "get working directory",
          commands: ["pwd"]
        }
      ]
    }
  ]
};
export default config;
