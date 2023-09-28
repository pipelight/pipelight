# Cast crate

## Role: convert Files into rust Structs

This crate makes use of serde to convert files into rust Structs.
In other words it **casts** files into Structs.

It converts:

- configuration files -> cast::Config;
- log files -> cast::Log.

## Usage

Load a Config or a Log struct from a filepath.

### Config

Loads logs from a file path.

```rs
let res = cast::Config::load("./public/pipelight.ts", None)?;
```

### Logs

Loads logs from a directory path.

```rs
let res = cast::Logs::load("./pipelight/logs", None)?;
```

The resulting cast structs are just dummy file conversion.
They are not to be used as is as they don't implement any useful methods.

The **cast** Structs are further converted into convenient **workflow** Structs.

File -> Cast::Pipeline -> Workflow::Pipeline
