# Pipelight

A cli to write routines in typescript/javascript.

```js
// tbh this tool is absurd! maybe my master peace!
// The last time I felt like it was this unfairly easy was when i used cosmiconfig.
// I swear, this is ridiculous.
```

## Install

```sh
paru -S pipelight
```

## Example

Create this config file at the root of your project

```ts
//pipelight.config.ts

const config = {
  pipelines: [
    name: "my_first_pipeline"
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

```sh
# This lists your pipelines
pipelight ls
```

Output:

```sh
status, last_run date, hook, name
```

Want more informations

```sh
# List your pipelines
pipelight ls -v

```

Verbosity tag can be combined with every command

```sh
# Trigger the pipeline
pipelight run my_first_pipeline

```

## The rules

As you may have seen or may not yet, pipelight folows some rules.

1. Your config file must be written in typescript/javascript and export
   an Object of type Config.
2. You can't retrigger a pipeline that is already running.

## Why another CICD tool ?

The need of something that keep it simple but allows for the great modularity.
As you know some javascript, you're ready to go.

Beware: You could end up using it for every repeted tasks be it a simple local routine, or an entreprise grade application deployement.

### The power of Typescript (Code as configuration)

This tool is written in Rust and Typescript.
It combines the speed and security of Rust with the easy scripting of Javascript.

Pipelight let you right your pipelines in typescript/javascript.
It allows _reusability_ and _flexibility_.

### Lightweight

It's written in Rust and exploit git hooks for speed and simplicity.

### Terminal friendly

Deploy, Backup, Restore... without living your terminal.
An efficient cli with pretty logs, and a great verbosity if needed.
