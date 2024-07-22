<span>
<h1>
<img width="125px" alt="pipelight_logo" src="https://pipelight.dev/images/pipelight.png"/>
<p>Pipelight - Tiny automation pipelines.</p>
</h1>
</span>

Automate your most boring and repetitive tasks.

- Define pipelines using **typescript, toml and yaml**.
- Pipelines get triggered on specific events (git hooks, file changes).

Checkout the [Documentation](https://pipelight.dev) for a much friendly approach
and a deeper understanding.

## Usage

<img width="480" alt="pipelight_demo" src="https://pipelight.dev/images/showcase.gif"/>

## 📦 A lightweight tool for CICD

Pipelight is a [Rust](https://www.rust-lang.org/) based small(13Mb) cli tool to
be used from inside a terminal.

## Define pipelines with a programming language

Fold your bash commands into an object `Pipeline{ Step{ Command }}` written in
**Typescript**, and it executes the pipeline on some events.

As long as you know javascript, you are ready to go with your favorite syntax
flavor 🍦.

Use a verbose and declarative syntax (Objects API).

```ts
const my_pipeline = {
  name: "build_my_website",
  steps: [
    {
      name: "clean directory",
      commands: ["rm -rf ./dist"],
    },
    {
      name: "build",
      commands: ["pnpm install", "pnpm lint", "pnpm build"],
    },
  ],
};
```

Use the provided sweet shorthands (Helpers API).

```ts
const my_pipeline = pipeline("build website", () => [
  step("clean directory", () => [`rm -rf ${build_dir}`]),
  step("build", () => ["pnpm install", "pnpm lint", "pnpm build"]),
  step("send to host", () => [`scp -r ${build_dir}`]),
  step("do stuffs on host", () => [
    ssh("host", () => ["systemctl restart nginx"]),
  ]),
]);
```

## Automatic triggers

Add automatic triggers to your pipeline.

- Run tests on file change,
- Deploy to production on push to master,...

```sh
# enable watcher and git hooks.
pipelight enable git-hooks
pipelight enable watcher
```

```ts
pipeline.add_trigger({
  branch: ["master"],
  actions: ["pre-push"],
});
```

## 🫦 Pretty and verbose logs

Get the pipeline most relevant informations or dive into the steps and commands
standard outputs.

Get the pipeline status, event, execution time... and more.

```sh
pipelight logs
```

<img width="500px" alt="pretty logs" src="https://pipelight.dev/images/log_level_error.png"/>

Get a tranparent outputs of every subprocesses.

```sh
pipelight logs -vvvv
```

<img width="500px" alt="pretty logs" src="https://pipelight.dev/images/log_level_trace.png"/>

## 🛠️ Install (Linux)

> [!IMPORTANT]  
> Pipelight is not (yet) compatible with macOS or Windows.

Checkout the
[instruction guide](https://pipelight.dev/introduction/install.html) for your
favorite package manager.

## 🚀 Get started!

Create a default configuration file `pipelight.ts` in your project root
directory with:

```sh
pipelight init
```

Try the harmless default pipeline:

```sh
pipelight run
```

And explore logs:

```sh
pipelight logs -vvvv
```

## Community

Reach the community whenever you need support or software improvements. On
[discord](https://discord.gg/swNRD3Xysz) or on telegram at
[@Areskul](https://t.me/areskul) or send a mail at areskul@areskul.com.

Licensed under GNU GPLv2 Copyright (C) 2023 Areskul

## Roadmap

Supported languages:

- [ ] feat: add support for apple pkl
- [ ] feat: add support for hashicorp hcl

Javascript helpers:

- [ ] feat: add helper for jucenit proxy
- [ ] refactor: rewrite docker helpers in rust wasm

Cli:

- [ ] feat: add a tui interface

Community:

- [ ] feat: add a news page on Documentation website
- [ ] refactor: move out of discord
