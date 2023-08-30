# ROADMAP

Pipelight has been designed with a final vision in mind.
Here are the features it needs to implement to reach v1.0.0

## core

- [ ] add: "blank" flag to trigger on server after a pipeline is done client side.
- [ ] add: signal handeling (for interupted piplines with Ctrl-C or SIGKILL/SiGTERM)
- [ ] add: More colorful and verbose internam error type (line, column + code sample)
- [ ] security: sanitize logs before reading (use pipeline.is_running method)
  - [ ]: remove unused internal logs on sanitization

## cli

- [ ] add: Clap version feature flag
- [ ] add: pipelight stop -> show a prompt with running pipeline names that can interupt.

## logs

- [ ] fix: better tree anchor spacing
- [ ] bug fix: handle corrupted logs EOF panic

## installer

- [ ] add: detect linux distribution or available package managers
- [ ] add: detect if deno is installed

## doc

- [ ] add: example in the source code.
- [ ] change: More appealing readme because the actual sucks!
- [ ] add: internal dependencies schema for better internals understanding
- [ ] add: process spawn strategy schema and justify choices (lightweight binaries)

## tests

- [ ] add: more cli tests.
- [ ] rewrite test for multiple bad config file.

# Issues that needs some brain cells to implement a solution

## The On_Abortion fallback issue

A way to trigger the `on_abortion` fallback after a hard kill would be:

- When cli is used after a pipeline abortion
- First sanitize log file
- Seek pipelines that have status running and compare to running processes
- Update logs and run the on_abortion hooks if any.
