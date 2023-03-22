import type { Config } from "npm:pipelight";
import { packagingPipeline } from "./.pipelight/config/packaging.ts";
const config: Config = {
  pipelines: [
    packagingPipeline,
    {
      name: "test",
      steps: [
        {
          name: "test",
          commands: ["cargo test --package pipeline"],
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

export default config;
