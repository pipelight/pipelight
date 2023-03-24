import type { Config, Pipeline, Step, Parallel } from "npm:pipelight";

const params = {
  distros: [
    {
      name: "debian",
      prefix: "deb",
      archive: "deb",
    },
    {
      name: "archlinux",
      prefix: "aur",
      archive: "pkg.tar.zst",
    },
    // {
    //   name: "fedora",
    //   prefix: "rpm",
    //   archive: "rpm",
    // },
  ],
};

const makePipeline = ({ distros }: any): Pipeline => {
  let pipeline: Pipeline = {
    name: "make:packages",
    steps: [],
  };
  let steps: Parallel = {
    parallel: [],
  };

  for (const { name, prefix } of distros) {
    steps.parallel.push(
      {
        name: `remove old ${name} container`,
        commands: [`docker container rm ${name}.latest `],
        non_blocking: true,
      },
      {
        name: `build ${name} container`,
        commands: [
          `sh -c \
          "cd ../ && docker build \
            --pull \
            --no-cache \
            -f pipelight/.pipelight/docker/Dockerfile.${prefix} \
            -t ${name}.latest ."`,
        ],
      },
      {
        name: `run ${name} container`,
        commands: [
          `docker run \
          --mount type=bind,source=./packages,target=/root/dist \
          --name="${name}.latest" \
          ${name}.latest
        `,
        ],
      }
    );
  }
  pipeline.steps.push(steps);
  return pipeline;
};

const packagingPipeline = makePipeline(params);
export { packagingPipeline };
