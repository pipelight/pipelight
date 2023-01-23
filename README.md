# Pipelight

A cli to write pipelines in javascript.
And trigger them automatically on git action.

Lightweight CI/CD tool.

## Install

From the AUR

```sh
paru -S pipelight
```

## Configuration example

Create a config file at the root of your project

```mjs
//pipelight.config.mjs
const config = {
  pipelines: [
    {
      name: "my_pipeline",
      steps: [
        {
          name: "list working directory",
          commands: ["ls -alh"],
        },
        {
          name: "get working directory",
          commands: ["pwd"],
        },
      ],
    },
  ],
};
export default config;
```

## Usage

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

Abort pipeline execution

```sh
pipelight stop my_pipeline
```

## Triggers

Only works in a Git repo.

```sh
git init
```

```mjs
//pipelight.config.mjs
const config = {
  pipelines: [
    {
      name: "automatic",
      triggers: [
        {
          actions: ["pre-push", "pre-commit"],
          branches: ["master"],
        },
      ],
    },
  ],
};
export default config;
```

Define triggers as combinations of branch name and git-hooks.

## Why another CICD tool ?

The need of something that keep it simple but allows for the great modularity.
As you know some javascript, you're ready to go.

### The power of Javascript (Code as configuration)

This tool is written in Rust and Typescript.
It combines the speed and security of Rust with the easy scripting of Javascript.

### Terminal friendly

Deploy, Backup, Restore... without living your terminal.
