const config = {
  pipelines: [
    {
      name: "default",
      steps: [
        {
          name: "wait",
          commands: ["ls", "sleep 30", "pwd"],
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
            "sleep 30",
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
  ],
};
export default config;
