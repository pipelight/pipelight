import type { Config, Pipeline } from "npm:pipelight";

const params = {
  distros: ["debian", "archlinux", "fedora", "centos"],
};

const makePipeline = ({ distros }: any): Pipeline => ({
  name: "test:packages",
  steps: [
    {
      name: "clean",
      commands: [`docker container rm debian.latest `],
      non_blocking: true,
    },
    {
      name: "build container",
      commands: [
        `sh -c \
          "cd ../ && docker buildx build \
          -f pipelight/.pipelight/docker/Dockerfile.debian \
          -t debian.latest ."`,
      ],
    },
    {
      name: "run container",
      commands: [
        `docker run \
          --name="debian.latest" \
          debian.latest
        `,
      ],
    },
  ],
});
const testPackagingPipeline = makePipeline(params);

export { testPackagingPipeline };
