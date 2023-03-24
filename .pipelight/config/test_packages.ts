import type { Config, Pipeline } from "npm:pipelight";

const params = {
  distros: [
    // {
    //   name: "debian",
    //   prefix: "deb",
    //   archive: "deb",
    // },
    {
      name: "archlinux",
      prefix: "aur",
      archive: "pkg.tar.zst",
    },
    {
      name: "fedora",
      prefix: "rpm",
      archive: "rpm",
    },
  ],
};

const makePipeline = ({ distros }: any): Pipeline => {
  let pipeline: Pipeline = {
    name: "test:packages",
    steps: [],
  };

  for (const { name, prefix } of distros) {
    pipeline.steps.push(
      {
        name: "clean old container",
        commands: [`docker container rm ${name}.latest `],
        non_blocking: true,
      },
      {
        name: "build container",
        commands: [
          `sh -c \
          "cd ../ && docker buildx build \
            -f pipelight/.pipelight/docker/Dockerfile.${prefix} \
            -t ${name}.latest ."`,
        ],
      },
      {
        name: "run container",
        commands: [
          `docker run \
          --mount type=bind,source=/packages,target=/root/dist \
          --name="debian.latest" \
          debian.latest
        `,
        ],
      }
    );
  }
  return pipeline;
};

const testPackagingPipeline = makePipeline(params);
export { testPackagingPipeline };
