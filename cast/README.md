# The Cast crate

It is a compatibility crate,
between pipelight core and configuration file formats.

## Convert files into struct

Uses serde to convert files into exploitable Structs.
(**Casts** files to structs)

It converts:

- configuration files -> cast::Config;
- log files into -> cast::Log.

The resulting cast struct are not to be used as is.
