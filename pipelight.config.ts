const config = {
  pipelines: [
    {
      name: "test",
      fr: "caca",
      steps: [
        {
          name: "mystep",
          commands: ["ls"],
        },
      ],
    },
    {
      name: "test_duplicate",
      fr: "caca",
      steps: [
        {
          name: "mystep",
          commands: ["ls"],
        },
      ],
    },
    {
      name: "test_two",
      fr: "caca",
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
