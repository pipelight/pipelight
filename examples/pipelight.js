const config = {
  pipelines: [
    {
      name: "test_kill",
      steps: [
        {
          name: `kill decendent subprocess`,
          commands: ["pwd", "sleep 10", "pwd"],
        },
      ],
    },
    {
      name: "test_tags",
      steps: [
        {
          name: `test tags`,
          commands: ["ls -l"],
        },
      ],
      triggers: [
        {
          tags: ["*"],
          actions: ["manual"],
        },
      ],
    },
    {
      name: "crago_tests",
      steps: [
        {
          name: "test",
          commands: ["cargo test"],
        },
      ],
    },
    {
      name: "test_parallel_modes",
      steps: [
        {
          parallel: [
            {
              name: "test",
              commands: ["llls"],
              mode: "continue",
            },
          ],
          mode: "continue",
        },
        {
          parallel: [
            {
              name: "test",
              commands: ["ls"],
            },
          ],
        },
      ],
    },
  ],
};

export default config;
