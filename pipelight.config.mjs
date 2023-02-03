const config = {
  pipelines: [
    {
      name: "default_para",
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
      triggers: [
        {
          branches: ["master", "dev"],
          actions: ["pre-push"],
        },
      ],
    },
    {
      name: "default",
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
          actions: ["pre-push", "manual"],
        },
      ],
    },
  ],
};
export default config;
