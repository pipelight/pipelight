import type { Config, Pipeline } from "https://deno.land/x/pipelight/mod.ts";
import { pipeline, step, parallel } from "https://deno.land/x/pipelight/mod.ts";
import { parse } from "https://deno.land/std/flags/mod.ts";

const flags = parse(Deno.args, {
  string: ["host"],
  default: {
    host: "localhost",
  },
});

const config: Config = {
  pipelines: [
    pipeline("test", () => [step(`test`, () => ["pwd"])]),
    pipeline("test_empty", () => [step(`launch a pipeline`, () => ["pwd"])]),
    pipeline("test_attached_pipelines", () => [
      step(`launch a pipeline`, () => [
        "cargo run --bin pipelight run test_rw --config test.pipelight.ts --attach",
      ]),
    ]),
    // Test watcher with a long running pipeline
    pipeline("test_watch", () => [
      step(`run harmless commands`, () => ["pwd", "sleep 30", "ls"]),
    ]).add_trigger({
      actions: ["watch"],
    }),
    pipeline("test_tags_trigger", () => [
      step(`test tags`, () => ["ls -l"]),
    ]).add_trigger({
      tags: ["*"],
      actions: ["manual"],
    }),
    pipeline("test_git_hooks(pre-push)", () => [
      step(`run harmless commands`, () => ["pwd", "sleep 30", "ls"]),
    ]).add_trigger({
      actions: ["pre-push"],
    }),
    pipeline("test_rw", () => [
      step(`kill decendent subprocess`, () => ["ppwd", "ls"]).set_mode(
        "jump_next"
      ),
      step(`kill decendent subprocess`, () => ["pwd", "ls", "sleep 10"]),
    ]),
    pipeline("test_long_running_pipeline", () => [
      step(`run and sleep`, () => ["pwd", "ls", "sleep 120", "pwd"]),
    ]),
    pipeline("test_deno_additional_arguments", () => [
      step(`host -> ${flags.host}`, () => ["cargo test --package pipeline"]),
    ]),
    pipeline("cargo_tests", () => [step("test", () => ["cargo test"])]),
    pipeline("test_parallel_modes", () => [
      parallel(() => [
        step("test", () => ["llls"]).set_mode("continue"),
      ]).set_mode("continue"),
      parallel(() => [step("test", () => ["ls"]).set_mode("continue")]),
    ]),
  ],
};

export default config;
