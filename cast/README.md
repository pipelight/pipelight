# Cast crate

## What is inside ?

Same as the **exec crate**, it is a compatibility crate,
between pipelight core and config file formats.

All the types and functions defined in this crate are to use to cast any documents
into a **Rust Pipelight Config struct**.
In other words it is just a crate to convert connfiguration files.

The supported configuration formats are:

- YAML
- TOML
- Javascript
- Typescript

It uses **serde** to check and convert provided config files.
The resulting struct `cast::Config` isn't optimized for furthere exploitation.

It is converted from a `cast::Config` into a `pipeline::Config`
inside the **pipeline crate** thus providing top level methods to inner structs like
`pipeline.run()`, `pipeline.stop()`.

## Files

You have an overview of the struct definition in `types.rs`.

## Tests

This is the crate where triggers array are converted. (sensible operation)
