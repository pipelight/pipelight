// test.pipelight.ts
// The every pipelines used for internal testing purpose

import type { Config, Pipeline } from "https://deno.land/x/pipelight/mod.ts";
import { parallel, pipeline, step } from "https://deno.land/x/pipelight/mod.ts";
import { parse } from "https://deno.land/std/flags/mod.ts";

const flags = parse(Deno.args, {
  string: ["host"],
  default: {
    host: "localhost",
  },
});

const config: Config = {
  options: {
    attach: true,
    log_level: "info",
  },

  pipelines: [
    // The simplest pipeline to run
    pipeline("test", () => [step(`test`, () => ["pwd"])]),

    // A pipeline that takes time to resolve
    pipeline("test_long_running_pipeline", () => [
      step(`run and sleep`, () => ["pwd", "ls", "sleep 120", "pwd"]),
    ]).add_trigger({
      actions: ["manual", "pre-push"],
      branches: ["dev"],
    }),

    // Run a pipeline that runs an attached pipeline
    pipeline("test_attached_pipelines", () => [
      step(`launch a pipeline`, () => [
        "cargo run --bin pipelight run test --config test.pipelight.ts --attach",
      ]),
    ]),

    // Watcher
    // Test watcher with a long running pipeline
    pipeline("test_watcher", () => [
      step(`run harmless commands`, () => ["pwd", "sleep 30", "ls"]),
    ]).add_trigger({
      actions: ["manual", "watch"],
    }),

    // Triggers
    // Test pipeline triggering by tags
    pipeline("test_tags_trigger", () => [step(`test tags`, () => ["ls -l"])])
      .add_trigger({
        tags: ["*"],
        actions: ["manual"],
      })
      .attach(),

    // Test pipeline triggering by git hook
    pipeline("test_git_hook", () => [
      step(`harmless commands`, () => ["pwd", "sleep 2", "ls"]),
    ])
      .add_trigger({
        actions: ["pre-push"],
      })
      .attach(),

    // Test setting modes and expected behaviors
    pipeline("test_rw", () => [
      step(`harmless commands`, () => ["ppwd", "ls"]).set_mode("jump_next"),
      step(`harmless commands`, () => ["pwd", "ls", "sleep 10"]),
    ]),

    // Run every unit tests
    pipeline("cargo_tests", () => [step("test", () => ["cargo test"])]),

    // Parallel
    pipeline("test_parallel_modes", () => [
      parallel(() => [step("test", () => ["llls"]).set_mode("continue")]),
      parallel(() => [step("test", () => ["ls"]).set_mode("continue")]),
    ]),

    // Set options
    pipeline("test_parallel_modes", () => [step("test", () => ["pwd"])]),
    {
      name: "test_options",
      steps: [step(`run harmless commands`, () => ["pwd", "sleep 2", "ls"])],
      triggers: [
        {
          actions: ["pre-push"],
        },
      ],
      options: {
        log_level: "trace",
      },
    },
  ],
};

export default config;
