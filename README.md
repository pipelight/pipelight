# Pipelight

A cli to write routines in typescript/javascript.

## Install

From the AUR

```sh
paru -S pipelight
```

## Example

Create this config file at the root of your project

```mjs
//pipelight.config.mjs
const config = {
  pipelines: [
    name: "my_pipeline"
    {
      name: "list working directory",
      commands: ["ls -alh"],
    },
    {
      name: "find a file",
      commands: ["find . -name myfile"],
    },
  ],
};
export default config;
```

List pipelines defined in config file

```sh
pipelight ls
```

or

```sh
pipelight ls -vvvv
```

Trigger a specific pipeline execution

```sh
pipelight run my_pipeline
```

Pretty print the pipeline status

```sh
pipelight logs
```

Verbosity can be increased

```sh
pipelight logs -vvvv
```

## Lint your config file

You can troubleshoot your config file with

```sh
node pipelight.config.mjs
```

## Why another CICD tool ?

The need of something that keep it simple but allows for the great modularity.
As you know some javascript, you're ready to go.

### The power of Javascript (Code as configuration)

This tool is written in Rust and Typescript.
It combines the speed and security of Rust with the easy scripting of Javascript.

### Terminal friendly

Deploy, Backup, Restore... without living your terminal.
