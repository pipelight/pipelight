# Pipeline crate

## What is inside ?

The pipeline crate is the biggest crate of the project.
Where all the magic happens.

## Getters

Contain getters for config, pipelines

# Config

Without a config file pipeline can only read logs.

Getting a config file is nearly mandatory.

Internally `Config::get()` will search for a config file
return it if founded,

- create git hooks if the file is at the root of a git directory.
- create log folders.
