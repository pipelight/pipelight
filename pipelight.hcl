# A pipeline
pipelines {
  name = "simple_example"
  steps {
    name     = "first"
    commands = ["ls", "pwd"]
  }
  steps {
    name     = "second"
    commands = ["ls", "pwd"]
  }
}
