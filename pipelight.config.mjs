const config = {
  pipelines: [
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
