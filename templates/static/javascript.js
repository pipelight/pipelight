const my_pipe = {
  name: "example",
  steps: [
    {
      name: "first",
      commands: ["ls", "pwd"],
    },
    {
      name: "second",
      commands: ["ls", "pwd"],
    },
  ],
};

const config = {
  pipelines: [my_pipe],
};

export default config;
