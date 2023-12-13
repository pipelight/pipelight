import type { Config, Pipeline } from "https://deno.land/x/pipelight/mod.ts";
import {pipeline, step } from "https://deno.land/x/pipelight/mod.ts";
import { parse } from "https://deno.land/std/flags/mod.ts";

const flags = parse(Deno.args, {
  string: ["host"],
  default: {
    host: "localhost",
  },
});

const config: Config = {
  pipelines: [
    pipeline("test", () => [step(`test`, () => ["pwd"])]).attach(),
    {
      name: "test_kill",
      steps: [
        {
          name: `kill decendent subprocess`,
          commands: ["pwd", "sleep 10", "pwd"],
        },
      ],
    },
    {
      name: "test_flags",
      steps: [
        {
          name: `host -> ${flags.host}`,
          commands: ["cargo test --package pipeline"],
        },
      ],
    },
    {
      name: "test_tags",
      steps: [
        {
          name: `test tags`,
          commands: ["ls -l"],
        },
      ],
      triggers: [
        {
          tags: ["*"],
          actions: ["manual"],
        },
      ],
    },
    {
      name: "crago_tests",
      steps: [
        {
          name: "test",
          commands: ["cargo test"],
        },
      ],
    },
    {
      name: "test_parallel_modes",
      steps: [
        {
          parallel: [
            {
              name: "test",
              commands: ["llls"],
              options: {
                mode: "continue",
              },
            },
          ],
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
