import type { Pipeline, Config } from "https://deno.land/x/pipelight/mod.ts";

const my_pipe: Pipeline = {
  name: "example",
  steps: [
    {
      name: "first",
      commands: ["ls", "pwd"],
    },
    {
      name: "second",
      commands: ["ls", "pwd"],
    },
  ],
};

const config: Config = {
  pipelines: [my_pipe],
};

export default config;
