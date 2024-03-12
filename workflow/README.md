# Workflow crate

## What is inside ?

One of the biggest crate of the project. Where all the magic behind pipeline
execution happens.

## Getters

Contain getters for config and pipeline.

# Config

`Config::get()` search for a config file return it if founded,

Init 

- create git hooks if at the root of a git repo or git bare repo.
- create log directories for internal usage.

# Cli crate

## What is inside ?

Here lays everything related to the cli.

- Commands definitions(`interface` directory)
- The actions to take on commands and options (`action.rs`)

- Autocompletion script generator (not mensionned in doc yet)

- **Special**: Traits to convert the clap.rs cli Commands back to original
  strings.

- **Special**: Process **self forking** strategy

### Process self-fork to background

It involved some imagination in its inner design, for the Pipeplight executable
to be align with the following requirements:

- keep runing in the background when terminal close;
- and remain a single executable to preserve types concistency.

This crate implements the most tricky part of Pipelight core.

When a command is launched `pipelight run my_pipe` some things happened under
the hood.

1. The pipelight executable runs and check the command in the foreground.
2. The executable duplicate itself to run a secondary executable in the
   backround.
3. The foreground executable dies.
4. The backround executable keeps running detached from terminal.
