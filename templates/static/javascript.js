// Create a pipeline with ObjectsAPI
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

// Create config
const config = {
  pipelines: [my_pipe],
};

// Export config object as default
export default config;
