import type {
  Config,
  Pipeline,
  Step,
  Parallel,
} from "https://deno.land/x/pipelight/mod.ts";

import $ from "https://deno.land/x/dax/mod.ts";

const version =
  (await $`git describe --tags --abbrev=0`.text()).replace("v", "") + "-1-any";
console.debug(version);

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
  {
    name: "fedora",
    prefix: "rpm",
    format: "rpm",
  },
];

const makePipeline = ({ name, prefix, format }: any): Pipeline => {
  let pipeline: Pipeline = {
    name: `package:${name}`,
    steps: [
      {
        name: `remove old ${name} container`,
        commands: [`docker container rm ${name}.latest `],
        mode: "jump_next",
      },
      {
        name: `build ${name} container`,
        commands: [
          `sh -c \
          "cd ../ && docker build \
            --pull \
            --no-cache \
            -f pipelight/.docker/Dockerfile.${prefix} \
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
            ./packages/pipelight-${version}.${format}
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
    name: "package:all",
    steps: [],
  };

  // Parallel pipeline execution
  const p: Parallel = {
    parallel: [],
  };
  for (const { name } of distros) {
    const step: Step = {
      name: `package:${name}`,
      commands: [`cargo run --bin pipelight run --attach package:${name} `],
    };
    p.parallel.push(step);
  }
  pipeline.steps.push(p);

  let uploadStep: Step = {
    name: `upload packages`,
    commands: [`cargo run --bin pipelight run --attach package:upload `],
  };
  pipeline.steps.push(uploadStep);
  // Update documentation .env
  let docStep: Step = {
    name: `update documentation`,
    commands: [`echo "VITE_GIT_VERSION=${version}" > ../doc.pipelight/.env`],
  };
  pipeline.steps.push(docStep);

  return pipeline;
};

const parallelPackagingPipeline: Pipeline = makeParallel(distros);

export { packagingPipelines, parallelPackagingPipeline };
