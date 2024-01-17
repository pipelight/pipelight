# Utils crate

## What is inside ?

Convenience functions

- Teleport
- Git
- Logger
- Date

## Logger

The logger is built on top of log4rs. Its on its way to be deprecated because it
provides not ernough flexibility.

Only use to log internals to stdout

### Git

Contains functions to:

- detect the git directory
- create and ensure pipelight git hooks

### Logger

Contains functions to:

- create and ensure a logger

### Teleport

This utility is greatly inspired by
[cosmiconfig](https://github.com/cosmiconfig/cosmiconfig).
`Teleport::search("filename")` seeks a file based on globbing
pattern you provided.

Contains functions to:

- Recursively search a file through the filesystem
- Change process cwd back and forth from the **pwd** and the **file parent
  directory**.


```rs

let mut portal = Teleport::new().preffix("pipelight");
portal.search()?;

// Teleport process to file path
portal.teleport();
// Teleport process to where it was originaly launched
portal.origin();
```

### Dates

Abstraction over date convertion and duration computation.

### Files

Abstraction over file reading, filepath (std::path::Path) usage...
