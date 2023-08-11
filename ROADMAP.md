# ROADMAP

Pipelight has been designed with a final vision in mind.
Here are the features it needs to implement to reach v1.0.0

## core

- [ ] add: pipelight stop -> prompt for running pipeline to interupt
- [ ] add: signal handeling (for interupted piplines with Ctrl-C or SIGKILL/SiGTERM)
- [ ] add: Clap version feature flag
- [ ] add: More colorful and verbose internam error type (line, column + code sample)
- [ ] security: sanitize logs before reading

## installer

- [ ] add: detect linux distribution or available package managers
- [ ] add: detect if deno is installed

## doc

- [ ] add: internal dependencies schema for better internals understanding
- [ ] add: process spawn strategy schema and justify choices (lightweight binaries)

## tests

- [ ] add: more cli tests.
- [ ] rewrite test for multiple bad config file.
