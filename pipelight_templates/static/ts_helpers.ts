import type { Config } from "https://deno.land/x/pipelight/mod.ts";
import { pipeline, step } from "https://deno.land/x/pipelight/mod.ts";

const my_pipe = pipeline("example", () => [
  step("first", () => ["ls", "pwd"]),
  step("second", () => ["ls", "pwd"]),
]);

const config = {
  pipelines: [my_pipe],
};

export default config;
