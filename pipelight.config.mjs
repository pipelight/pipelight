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
          name: "mystep",
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
  ],
};
export default config;
