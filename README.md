# Pipelight

A Lightweight CICD tool.

Write pipelines in Javascript.
And trigger them automatically on git action.

Define, Run pipe, check logs, without living your terminal.

[Full Documentation](https://doc.pipelight.areskul.com) in progress.

## What it is.

A Typescript bash wrapper.
A Rust program that execute "js strings parsed as bash commands" on a git event.
And return pretty logs.

...tries to be the same kind of software as Jenkins,Drone.io,Gitlab CICD

## Motivation

### Lazy

Config is written in Js so lots of loops and variables can be used
to end the struggle with CI/CD pipelines written in configuration optimised languages.

### Frugal Power User

I've been working with quite small servers, that struggle to build docker images, forget about kubernetes, graphana and so on.
But I have local powerful computers.
Pipelight allows me to git-push from a machine, build on another, and send the result on my tiny server, so I don't have to spend much money in Cloud ressources.

### Heavy work

When I need to deploy a machine, install and configure everything to deploy my apps in different envs..
I use it with docker, ansible, vagrant and others.
It becomes pretty simple to share variables/env between tools and create a one click full deployment.

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

If you're too "zero attention genZ tiktok user" to go further in the documentation.
Just read the [USAGE](#USAGE) section and rush to the CLI.
It will yell a few times until your config file is good to go.
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

Works better in a Git repo.

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

## Why another CICD tool ?

The need of something that keep it simple but allows for the great flexibility.

Pipelight does not use neither secrets nor plugins.
It directly loads your local environnement, so you can use your user ssh configuration, aliases and commands,
so you can easily couple it with Ansible, docker, Vagrant...

It takes Config as Code to another extend: Code as Config as Code!

### The power of Javascript (Code as configuration)

Javascrip is very good at writting object.
You can write functions in javascript to create multiple pipelines in a breeze.
Pipeline combines the speed and security of Rust with the easy scripting of Javascript.

## Why so fast ?

Pipelight is written in Rust and tightly coupled to linux and git.
It doesn't reinvent the wheel by making cumbersom event listeners, secrets or plugins.
Only git-hooks and bash commands with syntaxic sugar.

### Terminal friendly

Deploy, Backup, Restore... without living your terminal.
