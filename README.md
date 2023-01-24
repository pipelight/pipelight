# Pipelight

Now Stable!!

A cli to write pipelines in Javascript.
And trigger them automatically on git action.

Lightweight CI/CD tool.

[Full Documentation](https://pipelight.areskul.com) in progress.

## What it is.

A Rust program that execute "js strings parsed as bash commands" on a given git event.

## Motivation

### Money

I've been working with quite small servers, that struggle to build docker image, forget about kubernetes, graphana and so on.
But I have local powerful computers.
Pipelight allows me to git-push from a machine, build on another, and send the result on my tiny server.

### Lazy

You'll understand! ;D

## Install

From the AUR

```sh
paru -S pipelight
```

# Usage

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

List pipelines defined in config file

```sh
pipelight ls
```

or

```sh
pipelight ls -vvvv
```

---

## TL;DR

If you're too "zero attention genZ tiktok user" to go further in the documentation.
Just read until here and rush to the CLI.
It will yell a few times until your config file is good (don't forget to increase verbosity to debug).
But in the end it will run smooth.
Enjoy!

Come back later if you need some of the tips below or in the coockbook.

Plus Ultra!!

---

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

Works better in a Git repo.

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

Define triggers as combinations of branch-name and git-hooks.

## Types

The only constraint of pipelight is to "default export" an Object of type Config.
The second only constraint is that different Pipelines can't have the same name.

Here "?" means optionnal property in Typescript

```ts
type Config {
  pipelines?: [Pipeline]
}
type Pipeline {
  name: String, \\ Must be unique
  steps: [Step]
  triggers?: [Trigger]
}
type Step {
  command: [String]
}

struct Trigger {
  branches: [String],
  actions?: [Hook],
}

eum Hook {
  "pre-push",
  "pre-commit",
    ...
  // every git-hook
}
```

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

```mjs
//pipelight.config.mjs
const config = {
  pipelines: [
    {
      name: "deploy",
      steps: [
        {
          name: "test",
          commands: ["vitest"],
        },
        {
          name: "build",
          commands: ["vite build"],
        },
        {
          name: "send",
          commands: ["rsync local_files to_my_remote_server"],
        },
      ],
    },
  ],
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

# CookBook / Deployement startegies

!!! Work in progress !!!

## Tips

For the sake of reusability and when you need to deploy in multiple evironnements.

Overuse string interpolation!

```mjs
//pipelight.config.mjs
const params = {
  remote: {
    domain: "myserver.com",
    path: "/remote/directory",
  },
  local: {
    path: "/my/build/directory",
  },
};

const config = {
  pipelines: [
    {
      name: "deploy",
      steps: [
        {
          name: "send files to server",
          commands: [
            `scp -r ${params.local.path} ${params.remote.domain}@${params.remote.path}`,
          ],
        },
      ],
    },
  ],
};
export default config;
```

Overuse string interpolation, and parameter destructuring.

```mjs
//pipelight.config.mjs
const params = {
  remote: {
    domain: "myserver.com",
    path: "/remote/directory"

  },
  local: {
    path: "/my/build/directory"
  }
};

const makeConfig = ({remote, local}) = > {
  pipelines: [
    {
      name: "deploy",
      steps: [
        {
          name: `send files to ${remote.domain}`,
          commands: [
            `scp -r ${local.path} ${remote.domain}@${remote.path}`
          ],
        },
      ],
    },
  ],
};

const config = makeConfig(params)

export default config;
```

Overuse string interpolation, parameter destructuring and import/export ESM synthax.

Export here

```mjs
//.pipelight/config/default.mjs

const makeDefaultConfig = ({remote, local}) = > {
  pipelines: [
    {
      name: "deploy",
      steps: [
        {
          name: `send files to ${remote.domain}`,
          commands: [
            `scp -r ${local.path} ${remote.domain}@${remote.path}`
          ],
        },
      ],
    },
  ],
};

export {
  makeDefaultConfig
}

```

And import here

```mjs
//pipelight.config.mjs

import { makeDefaultConfig } from ".pipelight/config/default.mjs";

const params = {
  remote: {
    domain: "myserver.com",
    path: "/remote/directory",
  },
  local: {
    path: "/my/build/directory",
  },
};

const config = makeConfig(params);

export default config;
```

In the end it's just JS, either it is functionnal programming or object oriented,
you just have to return an object that satisfies the Config type.

## Dummy deployement

When you want to put stuffs from your computer to your server

```mjs
//pipelight.config.mjs
const config = {
  pipelines: [
    {
      name: "deploy",
      steps: [
        {
          name: "send files to server",
          commands: [
            "rsync local_files to_my_remote_server"
            "scp -r myfiles to_remote"
          ],
        },
      ],
    },
  ],
};
export default config;
```

## Server Side deployement

When you work in TEAM and want the server to deploy code.

### On your local

Creat a mirror repository.

```sh
git push --mirror ssh://username@mydomain.com/new-repository.git
```

### On your server(s)

Install pipelight on your server and adapt the hooks.

```mjs
//pipelight.config.mjs
      ...
      triggers: [
        {
          actions: ["pre-receive", "update", "post-receive"],
          branches: ["master"],
        },
      ],
```

## With remote Docker

Build docker images where the power resides, which mean locally, and not on remote tiny server.

```mjs
//pipelight.config.mjs
const params = {
  remote: "myremote.com"
  image: {
    name: "my_app",
    port:{
      in: 8080 ,
      out:80
    }
  }
}
const config = {
  pipelines: [
    {
      name: "deploy",
      steps: [
        {
          name: "build image",
          commands: [
            "rsync local_files to_my_remote_server"
            "scp -r myfiles to_remote"
          ],
        },
      ],
    },
  ],
};
export default config;
```
