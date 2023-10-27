// Get ressources from deno repo
import type { Pipeline, Config } from "https://deno.land/x/pipelight/mod.ts";

// Create a pipeline with ObjectsAPI
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

// Create config
const config: Config = {
  pipelines: [my_pipe],
};

// Export config object as default
export default config;
