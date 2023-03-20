import { Config } from "npm:pipelight";
const config: Config = {
  pipelines: [
    {
      name: "test",
      steps: [
        {
          name: "test",
          commands: ["cargo test --package pipeline -- --nocapture"],
        },
      ],
      triggers: [
        {
          branches: ["master", "dev"],
          actions: ["pre-push", "manual"],
        },
      ],
    },
  ],
};
// console.log(JSON.stringify(config));
export default config;
