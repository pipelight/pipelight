// Get ressources from deno repo
import type { Config } from "https://deno.land/x/pipelight/mod.ts";
import { pipeline, step } from "https://deno.land/x/pipelight/mod.ts";

// Create a pipeline
const my_pipe = pipeline("example", () => [
  steps("first", () => [
    "ls"
    "pwd"
  ]),
  steps("second", () => [
    "ls"
    "pwd"
  ])
]);

// Create config
const config = [
  pipelines: [ my_pipe ]
];

// Export config object as default
export default config;

