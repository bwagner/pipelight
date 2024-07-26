
# A pipeline
pipelines {

name =  "simple_example"

[[pipelines.steps]]
name = "list directory"
commands = ["ls"]

steps {
name = "get working directory"
commands = ["pwd"]

}

}

# Another pipeline

[[pipelines]]
name =  "simple_example2"

[[pipelines.steps]]
name = "list directory"
commands = ["ls"]

[[pipelines.steps]]
name = "get working directory"
commands = ["pwd"]

