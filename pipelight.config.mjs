const config = {
  pipelines: [
    {
      name: "test",
      steps: [
        {
          name: "mystep",
          commands: ["ls", "sleep 35", "pwd"],
        },
      ],
      triggers: [
        {
          branches: ["master"],
          actions: ["pre-push"],
        },
      ],
    },
  ],
};
export default config;
