# Internal API

Pipelight has well matured since its first prototype in 2021 (Simpcicd fully written in typescript/nodejs).
The command line interface won't change as often as before and as the number of users grows,
the software is getting more and more structured, tested and documented.

I feel it is now time for the internal functionning to be explained with schemas and all.
Here will be a quick overview of the crate, functions and global variables roles and interdependencies explanation.

## Functionning

When running a pipeline.
This happens.

Read config file -> Create a Inner Object (Pipeline struct) -> Run processes in the defined order while logging.

### Configuration file reading

The purpose of the first set of functions called when executing a CLI command,
is to find, read and parse the pipelight configuration file.

### Find

The Teleport crate is a kind of [cosmiconfig](https://github.com/cosmiconfig/cosmiconfig).
You call `Teleport::search("filename")` and it recursively seeks a file based on globbing pattern you provided.

The advantages it provide is that it can change the process cwd to where the configuration file lays and change it again to the previous cwd.

This is a back and forth teleportation to the folder that contain our file of interest.

```rs
let mut portal = Teleport::new().preffix("pipelight");
portal.search()?;

// Teleport process to file path
portal.teleport();
// Teleport process to where it was originaly launched
portal.origin();

```

**The Teleport crate may be the first internal crate to be publicly released to crate.io.**

### Read and Parse
