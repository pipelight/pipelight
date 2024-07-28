# A pipeline
pipelines {
  name = "simple_example"
  steps {
    name     = "list directory"
    commands = ["ls"]
  }
  steps {
    name     = "get working directory"
    commands = ["pwd"]
  }
}

# Another pipeline
pipelines {
  name = "simple_example2"
  steps {
    name     = "list directory"
    commands = ["ls"]
  }
  steps {
    name     = "get working directory"
    commands = ["pwd"]
  }
}
