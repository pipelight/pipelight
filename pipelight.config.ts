// import { Config } from "./typescript/src/mod.ts";
// const config: Config = {
const config = {
  pipelines: [
    {
      name: "simple_example_ts",
      steps: [
        {
          name: "list files",
          commands: [
            "ls -l",
            `ls  \
            -al
            `,
          ],
        },
        {
          name: "get working directory",
          commands: ["pwd"],
        },
      ],
    },
    {
      name: "example_w_fallback",
      steps: [
        {
          name: "commands with typo",
          commands: ["llls"],
          on_failure: [
            {
              name: "wait failure",
              commands: [
                `echo 
                  \"On_failure fallback executed
                  yes
                  \"
                `,
              ],
            },
          ],
        },
        {
          name: "get working directory",
          commands: ["pwd"],
        },
        {
          name: "wait",
          commands: ["sleep 10"],
        },
      ],
    },
    {
      name: "example_parallel_steps",
      steps: [
        {
          parallel: [
            {
              name: "wait",
              commands: ["ls", "sleep 15", "pwd"],
            },
            {
              name: "wait2",
              commands: ["ls", "sleep 15", "pwd"],
            },
          ],
        },
      ],
    },
    {
      name: "posthook",
      steps: [
        {
          name: "wait",
          commands: ["ls", "sleep 1", "psssswd", "pwd"],
          on_failure: [
            {
              name: "wait failure",
              commands: [`echo \"caca\"`],
            },
          ],
        },
        {
          name: "wait2",
          commands: ["ls", "sleep 1", "pwd"],
        },
        {
          name: "wait3",
          commands: ["ls", "sleep 1", "pwd"],
        },
      ],
      triggers: [
        {
          branches: ["master", "dev"],
          actions: ["pre-push"],
        },
      ],
    },
    {
      name: "test",
      steps: [
        {
          name: "test",
          commands: [
            "cargo test --package pipeline -- --nocapture --test-threads=1",
          ],
        },
      ],
      triggers: [
        {
          branches: ["master", "dev"],
          actions: ["pre-push", "manual"],
        },
      ],
    },
    {
      name: "sync",
      steps: [
        {
          name: "run another pipeline",
          commands: ["cargo run --bin pipelight run test --attach"],
        },
      ],
      triggers: [
        {
          branches: ["master", "dev"],
        },
      ],
    },
  ],
};
export default config;
