# Pipelight utils crate

!! API not stabilized and lacking documentation !!
!! Download at your own risks !!

A set of trivial utilities for command line tools.

These are the most cleaned up code modules from
[pipelight](https://github.com/pipelight/pipelight)
an automation cli.

## Breakin Changes.

- v0.2.9

  The process management module has been moved into its own crate
  [pipelight_exec](https://crates.io/crates/pipelight_exec)

  replace

  ```rust
  use pipelight_utils::exec::Process;

  ```

  with

  ```rust
  use pipelight_exec::Process;

  ```
