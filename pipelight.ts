import type { Config, Pipeline } from "https://deno.land/x/pipelight/mod.ts";
import {
  packagingPipelines,
  parallelPackagingPipeline,
} from "./.pipelight/config/packages.ts";
import { uploadPipeline } from "./.pipelight/config/upload.ts";

import { parse } from "https://deno.land/std/flags/mod.ts";
const flags = parse(Deno.args, {
  string: ["host"],
});

const config: Config = {
  pipelines: [
    parallelPackagingPipeline as Pipeline,
    ...packagingPipelines,
    uploadPipeline,
    {
      name: "test_flags",
      steps: [
        {
          name: `host -> ${flags.host}`,
          commands: ["cargo test --package pipeline"],
        },
      ],
      triggers: [
        {
          branches: ["dev", "feature/*"],
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
    {
      name: "test-para-mode",
      steps: [
        {
          parallel: [
            {
              name: "test",
              commands: ["llls"],
              mode: "continue",
            },
          ],
          mode: "continue",
        },
        {
          parallel: [
            {
              name: "test",
              commands: ["ls"],
            },
          ],
        },
      ],
    },
  ],
};

export default config;
