// This is a template config file
// Feel free to test the cli NOOOOW,
// as non of this commands will arm your computer!
const config = {
  pipelines: [
    {
      name: "test",
      steps: [
        {
          name: "mystep",
          commands: ["ls", "sleep 8", "pwd"],
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
