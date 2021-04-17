# Run

Starts a shell according to ya [configuration](./config).

```yaml
USAGE:
    ya shell [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <config>    Location of configuration file [default: .config/ya/ya.yml]
```

The shell command automates actions taken to start an interactive shell.

## Ya Config Syntax

```yaml
config:
  shell:
    plugin: shell
    config:
      shell: bash
```

This command, for example, will execute the command `bash` and forward stdin from `ya`, providing a `bash` shell.

### Shell

Any command that provides an interactive session can be used.

```yaml
config:
  shell:
    plugin: shell
    config:
      shell: python3
```

### This Project's Configurations

```yaml
config:
  shell:
    plugin: shell
    config:
      command: |
        if [[ "$(docker images -q ya-builder 2> /dev/null)" == "" ]]; then
          ya build
        fi
        docker run -it --rm -v $PWD:/app ya-builder
```

At the time of writing, this is the shell config of this project.

Associated with this project is a [Dockerfile](/.config/docker/Dockerfile). That Dockerfile installs for all the utilities required to work with this project.

Note that this config has a dependency on `ya build`.

It starts a `bash` shell within a container created through the `build` command and forwards stdin to that session.
