# Run

```yaml
USAGE:
    ya run [FLAGS] [OPTIONS] [arguments]...

FLAGS:
    -h, --help            Prints help information
    -n, --no-arguments    Don't parse arguments e.g. $@
    -V, --version         Prints version information

OPTIONS:
    -c, --config <config>    Location of configuration file [default: .config/ya/ya.yml]

ARGS:
    <arguments>...    Optional arguments to pass into run command
```

The run command is designed to provide a way to automate a specific action taken to interact with your project.

## Ya Config Syntax

```yaml
config:
  run:
    plugin: shell
    config:
      command: echo hello
```

This command, for example, will execute the command `echo hello` within a child `bash` shell.

### Shell

You can adjust the shell used for the command by specifying a value for the shell key.

```yaml
config:
  run:
    plugin: shell
    config:
      shell: python3
      command: print("hello")
```

### Arguments

Pass arguments from the command line like this:

```yaml
config:
  run:
    plugin: shell
    config:
      command: echo "hello $@"
```

This allows you to add arguments to your run command:

```bash
$ ya run my friend!
hello my friend!
```

### Replacement Key

If you don't want `$@` to be the string that captures input from the user, you can change it like so:

```yaml
config:
  run:
    plugin: shell
    config:
      command: echo "hello <target_of_salutation>"
      argument_replacement_key: <target_of_salutation>
```

### Complexity

Adjust the complexity of your run command according to the complexity of the tasks you need to perform with that configuration file.

```yaml
config:
  run:
    plugin: shell
    config:
      command: |
        if [[ "$(docker images -q ya-builder 2> /dev/null)" == "" ]]; then
          ya build
        fi

        docker run -t --rm -v $PWD:/app ya-builder -c "$@"
```

At the time of writing, this is the run config of this project.

Associated with this project is a [Dockerfile](/.config/docker/Dockerfile). That Dockerfile installs for all the utilities required to work with this project.

Note that this config has a dependency on `ya build`.

Build commands differ from run commands in that they are designed to have verifiable artifact(s) produced as a consequence of their command.
