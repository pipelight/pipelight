import type { Config, Pipeline } from "https://deno.land/x/pipelight/mod.ts";
import {
  packagingPipelines,
  parallelPackagingPipeline,
} from "./.pipelight/config/packages.ts";
import { uploadPipeline } from "./.pipelight/config/upload.ts";

const config: Config = {
  pipelines: [
    parallelPackagingPipeline as Pipeline,
    ...packagingPipelines,
    uploadPipeline,
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
          branches: ["feature/*"],
          actions: ["manual"],
        },
        // {
        //   tags: ["*"],
        //   actions: ["manual"],
        // },
      ],
    },
    {
      name: "test-dev",
      steps: [
        {
          name: "test",
          commands: ["cargo test"],
        },
      ],
      triggers: [
        {
          branches: ["dev"],
          actions: ["pre-push", "manual"],
        },
      ],
    },
  ],
};

export default config;
