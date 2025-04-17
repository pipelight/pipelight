# Roadmap

## To do next

## Multilang pipelines (planed for june 2025)

Currently, pipelight loads the first file it recursively finds.

I would like pipelight to load every file in can recursively find.
So that a user can define pipelines in toml, yaml...
side by side in the same directory.

Refactor configuration loader.
Nothing too difficult

## Live execution (not planned yet)

A bit more difficult.

- Need to refresh the screen every time a process output is pushed to kernel queue.

We need to use the kernel here because implementing our own queue could lead
to read/write concurrency on process input/output.

# Feat `no-log`

Add a _no-log_ cli parameter to run a pipeline silently
without producing any logs on filesystem.
