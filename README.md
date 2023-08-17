<span>
<h1>
<img width="125px" alt="pipelight_logo" src="https://pipelight.dev/images/pipelight.png"/>
<p>Pipelight</p>
</h1>
<p>Automation pipelines but easier.</p>
</span>

## Join the Community

### Contacts

Join the **[Discord server](https://discord.gg/swNRD3Xysz)**

### Documentation

Checkout the [Full Documentation](https://pipelight.dev)

## Define pipelines

Alongside your code in a `pipelight.ts` file.

Simple pipelines with Toml and Yaml.
Complex pipelines with Typescript.

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

And much more to come in the Cookbook.

## Add triggers

Automatic triggers in git repo.

```ts
pipeline.add_trigger({
  tags: "v*",
  action: "pre-push",
});
```

## Enjoy the CLI

Manual triggers via the CLI

```sh
pipelight run
```

## Pretty logs

```sh
pipelight logs
```

<img width="500px" alt="pretty logs" src="https://pipelight.dev/images/example_log_level_4.png"/>

Licensed under GNU GPLv2
Copyright (C) 2023 Areskul
