import type { Config, Pipeline, Step, Parallel } from "npm:pipelight";

const version = "git describe --tags --abbrev=0 | sed s/v//" + "-1";
const distros = [
  {
    name: "debian",
    prefix: "deb",
    format: "deb",
  },
  {
    name: "archlinux",
    prefix: "aur",
    format: "pkg.tar.zst",
  },
  // {
  //   name: "fedora",
  //   prefix: "rpm",
  //   archive: "rpm",
  // },
];

const makePipeline = ({ name, prefix, format }: any): Pipeline => {
  let pipeline: Pipeline = {
    name: `package:${name}`,
    steps: [
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
          --name="${name}.latest" \
          ${name}.latest
        `,
          `docker cp \
            ${name}.latest:/root/dist/pipelight.${format} \
            ./packages/pipelight.${version}.${format}
        `,
        ],
      },
    ],
  };
  return pipeline;
};

const packagingPipelines: Pipeline[] = [];
for (const params of distros) {
  packagingPipelines.push(makePipeline(params));
}

const makeParallel = (distros: any[]): Pipeline => {
  const pipeline: Pipeline = {
    name: "make:packages",
    steps: [],
  };

  // Parallel pipeline execution
  const p: Parallel = {
    parallel: [],
  };
  for (const { name } of distros) {
    const step: Step = {
      name: `package:${name}`,
      commands: [` pipelight run --attach package:${name} `],
    };
    p.parallel.push(step);
  }
  pipeline.steps.push(p);

  // Update documentation .env
  const step: Step = {
    name: `update documentation`,
    commands: [`echo "VITE_GIT_VERSION=${version}" >> ../doc.pipelight/.env`],
  };

  return pipeline;
};

const parallelPackagingPipeline: Pipeline = makeParallel(distros);

export { packagingPipelines, parallelPackagingPipeline };
