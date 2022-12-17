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
          commands: ["sleep 6", "ls"],
        },
      ],
    },
    {
      name: "test_duplicate",
      steps: [
        {
          name: "mystep",
          commands: ["ls", "echo $0"],
        },
      ],
    },
    {
      name: "test2",
      steps: [
        {
          name: "mystep",
          commands: ["ls"],
        },
      ],
    },
  ],
};
export default config;
