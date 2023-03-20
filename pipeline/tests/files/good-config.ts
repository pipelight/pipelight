const config = {
  pipelines: [
    {
      name: "default",
      steps: [
        {
          name: "wait",
          commands: ["ls", "sleep 15", "pwd"],
        },
      ],
    },
  ],
};
export default config;
