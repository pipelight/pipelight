import type { Config, Pipeline } from "npm:pipelight";
import {
  packagingPipelines,
  parallelPackagingPipeline,
} from "./.pipelight/config/packages.ts";
const config: Config = {
  pipelines: [
    parallelPackagingPipeline as Pipeline,
    ...packagingPipelines,
    {
      name: "yes",
      steps: [
        {
          name: "yes",
          commands: ["yes"],
        },
      ],
    },
    {
      name: "test",
      steps: [
        {
          name: "cargo test",
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
