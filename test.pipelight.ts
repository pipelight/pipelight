import type { Config, Pipeline } from "https://deno.land/x/pipelight/mod.ts";
import { parse } from "https://deno.land/std/flags/mod.ts";

const flags = parse(Deno.args, {
  string: ["host"],
  default: {
    host: "localhost",
  },
});

const config = {
  pipelines: [
    {
      name: "test_watch",
      steps: [
        {
          name: `kill decendent subprocess`,
          commands: ["pwd", "ls"],
        },
      ],
      triggers: [
        {
          actions: ["watch"],
        },
      ],
    },
    {
      name: "test_rw",
      steps: [
        {
          name: `kill decendent subprocess`,
          commands: ["ppwd", "ls"],
          mode: "jump_next",
        },
        {
          name: `kill decendent subprocess`,
          commands: ["pwd", "ls", "sleep 10"],
        },
      ],
    },
    {
      name: "test_kill",
      steps: [
        {
          name: `kill decendent subprocess`,
          commands: ["pwd", "ls", "sleep 120", "pwd"],
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
