# Pipelight

Self-hosted automation pipelines!

<img width="100px" alt="ferris" src="https://pipelight.dev/images/ferris_playing_pipelight.png"/>

## Define pipelines

Alongside your code in a `pipelight.ts` file.

Simple and complex pipeline definition.

### Enjoy the declarative Object syntax.

```ts
const pipeline: Pipeline = {
  name: "build website",
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

### Taste a bunch of Typescript Helpers.

```ts
pipeline("deploy_to_remote", () => [
  step("build_new_images", () => docker.update()),
  step("send_images_to_remote", () => docker.images.send([host])),
  step("upgrade_remote_containers", () => ssh([host], docker.upgrade())),
]);
```

## Add Triggers

Automatic triggers in git repo.

```ts
pipeline.add_trigger({
  tags: "v*",
  action: "pre-push",
});
```

## Enjoy the Cli

Manual triggers via the CLI

```sh
pipelight run
```

Pretty logs

```sh
pipelight logs
```

<img width="400px" alt="pretty logs" src="https://pipelight.dev/images/example_log_level_4.png"/>

And more to come in the Cookbook.

[Full Documentation](https://pipelight.dev)

<img width="200px" alt="pipelight logo" src="https://pipelight.dev/images/pipelight.png"/>

Licensed under GNU GPLv2
Copyright (C) 2023 Areskul
