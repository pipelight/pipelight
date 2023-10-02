import type { Config, Pipeline } from "https://deno.land/x/pipelight/mod.ts";
import {
  packagingPipelines,
  parallelPackagingPipeline,
} from "./cicd/packages.ts";
import { uploadPipeline } from "./cicd/upload.ts";

const config: Config = {
  pipelines: [
    parallelPackagingPipeline as Pipeline,
    ...packagingPipelines,
    uploadPipeline,
    {
      name: "test",
      steps: [
        {
          name: "get pwd",
          commands: ["pwd"],
        },
      ],
      triggers: [
        {
          branches: ["dev"],
          actions: ["watch", "pre-push", "manual"],
        },
      ],
    },
  ],
};
export default config;
