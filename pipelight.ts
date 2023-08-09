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
          name: "get pwd",
          commands: ["pwd"],
        },
      ],
      triggers: [
        {
          branches: ["dev"],
          actions: ["pre-push"],
        },
      ],
    },
  ],
};
export default config;
