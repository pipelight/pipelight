import type { Config, Pipeline } from "https://deno.land/x/pipelight/mod.ts";
import { parse } from "https://deno.land/std/flags/mod.ts";

const config = {
  pipelines: [
    {
      name: "test_empty",
      steps: [
        {
          name: `launch a pipeline`,
          commands: ["pwd"],
        },
      ],
    },
  ],
};
export default config;
