# Order

1. Use &str instead of String when possible
2. Unit Tests
3. Hooks creation (ln -s) file! macro usage to know wich event triggered pipe

# Exec

Create run binary (cli + trigger pipeline execution in child process)
Use "process" crate instead of subprocess for mac and windows compatibility

# Typescript

## Type check

Configure eslint for type reporting.
No need for custom logging.
Native logger is already well suited?

Print **wrapped lines** after error

# Rust

## Verbosity

Set normal level to none.
Set log4rs verbosity levels via the cli

## Logger

print back json to colorful pretty log

# Cli

Set autocompletion and autocorrection (for bash and zsh)
