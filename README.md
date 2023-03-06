# Pipelight

A tiny automation tool.

Wrap your bash script with Typescript.
And trigger those pipelines automatically on git action.

Define, Run pipe, check Logs, without living your terminal.

[Full Documentation](https://doc.pipelight.areskul.com)

## Install

Package only available on Arch linux.
(Available soon on Debian/Ubuntu and Fedora)

Install from the AUR

```sh
paru -S pipelight
```

Or from source

```sh
git clone <this_repo>
cd pipelight
cargo build --release
cp target/release/pipelight* /<my_bin_directory>/
```

---

## TL;DR

If you're too can not stand the suspens and go further in the documentation..
Just read the [USAGE](#USAGE) section, install and try the Cli.
It will yell a few times until your config file is good.
But in the end it will run smooth.
Enjoy!

In short:
Pipelight is easy to install, fast, and usable for every kind of project.

---

# Usage

## Configuration example

Create a config file at the root of your project

```ts
//pipelight.config.ts
const config = {
  pipelines: [
    {
      name: "my_pipeline",
      steps: [
        {
          name: "list working directory",
          commands: ["ls -alh"]
        },
        {
          name: "get working directory",
          commands: ["pwd"]
        }
      ]
    }
  ]
};
export default config;
```

## Command Line Interface (Cli)

In the same folder..

List pipelines defined in config file

```sh
pipelight ls
```

or

```sh
pipelight ls -vvv
```

Run a pipeline

```sh
pipelight run <pipeline_name>
```

Compulsively check execution with pretty termial logs

```sh
pipelight logs
```

Verbosity can be increased..

```sh
pipelight logs -vvv
```

<p align="center">
  <img class="terminal" src="https://doc.pipelight.areskul.com/images/log_level3.png" alt="pretty verbose logs picture">
</p>
_The actulal pipeline to deploy the documentation website._

Abort pipeline execution

```sh
pipelight stop <pipeline_name>
```

## Triggers

Only works in a Git repo.

```ts
//pipelight.config.ts
const config = {
  pipelines: [
    {
      name: "automatic",
      triggers: [
        {
          actions: ["pre-push", "pre-commit"],
          branches: ["master"]
        }
      ]
    }
  ]
};
export default config;
```

Define triggers as combinations of branch-name and git-hooks.

## How it works

Think of it as a bash wrapper.

When we first deploy a project we quickly edit some raw bash scripts.
It's clearly the fastest way to test.

```sh
//deploy.sh
vitest
vite build
rsync local_files to_my_remote_server
```

But at some point, this method lakes verbosity, and automation...
Just put your commands into a Pipeline object.

```ts
//pipelight.config.ts
import { Config } from "npm:pipelight";
const config: Config = {
  pipelines: [
    {
      name: "deploy",
      steps: [
        {
          name: "test",
          commands: ["vitest"]
        },
        {
          name: "build",
          commands: ["vite build"]
        },
        {
          name: "send",
          commands: ["rsync local_files to_my_remote_server"]
        }
      ]
    }
  ]
};
export default config;
```

Add triggers, appreciate logs, and bettern your deployment scripts.
