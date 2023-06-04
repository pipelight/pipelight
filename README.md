# Pipelight

Self-hosted automation pipelines!

# Define pipelines

Alongside your code in a `pipelight.ts` file.

Simple and Complex pipeline definition thanks to Typescript.

Enjoy the declarative Object syntax.

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

Taste a bunch of Typescript Helpers.

```ts
pipeline("deploy_to_remote", () => [
  step("build_new_images", () => docker.update()),
  step("send_images_to_remote", () => docker.images.send([host])),
  step("upgrade_remote_containers", () => ssh([host], docker.upgrade())),
]);
```

Automatic triggers in git repo.

```ts
pipeline.add_trigger({
  tags: "v*",
  action: "pre-push",
});
```

Manual triggers via the CLI

```sh
pipelight run
```

Pretty logs

```sh
pipelight logs
```

And more to come in the Cookbook.

[Full Documentation](https://pipelight.dev)

Licensed under GNU GPLv2
Copyright (C) 2023 Areskul
