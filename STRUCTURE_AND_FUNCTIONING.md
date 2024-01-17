# Internal structure

The pipelight source code is splitted into 5-6 crates.

You can find READMEs in each modules(crates) for better crate by crate
understanding.

## pipelight crate

**Entry point**

This crate is the smallest and has only one purpose. It is the entry point for
building a single binary.

## cast crate

**File convertion utilities**

Mostly a wrapper around [serde](https://github.com/serde-rs/serde). This crate
contains the functions to read and parse `pipelight.<toml/yaml/ts>`
configuration files into Rust simple structs.

Beware! Those structs are not to be used as is and or converted to more
comfortable structures.

## utils crate

**Trivial convenience tooling**

Contains great utility functions for some trivial things.

- git (fetch somoe info on git repositpories and generate git-hooks)
- logger (set the logger configuration on the fly)
- teleport (find a file recursively in the filesystem)
- dates (Date manipulation and serialization, Time computing)
- files (Query the filesystem)
- signal (functions to handle SIGTERM, Ctrl-C and others)

## template crate

**Generate config file templates**

Contains the functions to generate templates when using commands like
`pipelight init --template toml`

## exec crate

**Wrapper around process execution**

Functions to pipe processes output, retrieve pid, cwd and so on.

## switch crate

**A switch/case over commands line args**

Determines which functions to run based on the command line arguments.

## cli crate

**Command line structure definition**

Define the command line sub-command, options and arguments. Makes extensive use
of [Clap.rs](https://docs.rs/clap/latest/clap/)

## workflow crate

**The pipeline execution logique***

Read the Pipeline Struct extracted from tthe configuration file and set the
execution workflow.

# Internal functionning

**When running a pipeline. This is what happens.**

The entry point is `pipelight/src/main.rs`.

Then a switch/case function from `swicth/src/case,rs` to determine wich command
to execute.
