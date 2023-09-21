// Get ressources from deno repo
import type { Config } from "https://deno.land/x/pipelight/mod.ts";
import { pipeline, step } from "https://deno.land/x/pipelight/mod.ts";

// Create a pipeline with HelpersAPI
const my_pipe = pipeline("example", () => [
  step("first", () => ["ls", "pwd"]),
  step("second", () => ["ls", "pwd"]),
]);

// Create config
const config = {
  pipelines: [my_pipe],
};

// Export config object as default
export default config;
