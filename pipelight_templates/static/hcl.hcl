pipelines = [{
  name = "simple_example"
  steps = [{
      name     = "list directory"
      commands = ["ls"]
    },
    {
      name     = "get working directory"
      commands = ["pwd"]
  }]
}]
