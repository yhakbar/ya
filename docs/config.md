# Ya YAML Spec

## Spec

```yml
simple_command: String

full_command:
  prog: String
  args: List of Strings
  cmd: Optional String
  chdir: Optional String

from_command:
  from: String
  cmd: Optional String

sub_commands:
  sub:
    simple_sub_command: String
    from_sub_command:
      from: String
      cmd: Optional String
    full_sub_command:
      prog: String
      args: Optional List of Strings
      cmd: Optional String
      chdir: Optional String
    sub_sub_commands:
      sub:
        simple_sub_sub_command: String
```

Each command above represents a different way to define a command in a `ya` config file.

### Simple Commands

- `simple_command` is a string that will be run as a command. This is equivalent to `full_command` with `prog` set to `bash`, `args` set to `["-c"]` and `cmd` set to the value of the string.

### Full Commands

- `full_command` is a mapping defining multiple configurations for a command.
  - `chdir` is the optional directory to change to before running the command.
  - `prog` is the program to run. Default: `bash`.
  - `args` is a list of arguments to pass to the program. Default: `["-c"]`. Can be set to `[]` to pass no arguments.
  - `cmd` is the optional command to run. To be added to the command after the program and arguments.

### From Commands

- `from_command` is a mapping defining a command to run from another config file.
  - `from` is a configuration file to use instead of the current one for a given command.
  - `cmd` is the optional selection of a specific command in the `from` config file. By default this is the command being run.

### Sub Commands

- `sub_commands` is a mapping with a key of `sub` that defines multiple subcommands. Each of the commands defined in the `sub` mapping can be defined in the same way as the root commands for the config file. Simple Commands, Full Commands, From Commands, and Sub Commands can all be defined in the `sub` mapping.

## Config File Precedence

Ya will look for a config file in the following locations, in order:

- `./ya.yml`
- `./ya.yaml`
- `./.config/ya.yml`
- `./.config/ya.yaml`
- `$GIT_ROOT/ya.yml`
- `$GIT_ROOT/ya.yaml`
- `$GIT_ROOT/.config/ya.yml`
- `$GIT_ROOT/.config/ya.yaml`

Where `$GIT_ROOT` is the root of the git repository that the current directory is in, if it is in one.

Note that although the highest precedence config file is the one in the current directory, I recommend tucking your config file into a `.config` directory when you can. This reduces clutter in your repo and allows you to quickly override the config file by creating a temporary config in the current directory.

You can also explicitly specify a config file with the `-c`/`--config` flag or by setting the `YA_CONFIG` environment variable with the path to the config file. Note that if both are set, the flag will take precedence.

Note that the `YA_CONFIG` is used to simplify the process of discovering config files when calling `ya` recursively. For example, if you have a config file in a non-default location, and use `-c` to specify it, the default behavior of `ya` will be to re-use that config file.

## Building a Config

Ya does not come with any default commands. You must build your own config file to use it. The config file is a YAML file that contains a mapping of commands to run. The keys are the names of commands and the values are the commands to run.

Example config files can be found in the [examples](/examples) directory.

### Simple Commands Cont’d

The simplest config file would look something like this:

```yml
run: echo "Hey ya!"
```

It's a simple command called `run` with a value of `echo "Hey ya!"`. By default, `ya` will run the command `bash -c` followed by the value of a key in the config file. In this case, the following invocation of `ya`:

```bash
❯ ya run
Hey ya!
```

Would be equivalent to running:

```bash
❯ bash -c "echo \"Hey ya!\""
Hey ya!
```

You can use this if you have a one-liner that you would like to save to re-use more easily.

Note that you can also use the multi-line string syntax in YAML to make longer scripts usable as well:

```yml
run: |
  echo "This"
  echo "is"
  echo "a"
  echo "multi-line"
  echo "script"
```

```bash
❯ ya run
This
is
a
multi-line
script
```

### Full Commands Cont’d

The full configurations that are available to a command provide more flexibility. For example, you might want to run a command that has multiple arguments. You can do that like so:

```yml
install:
  prog: cargo
  args: ["install", "--path", "."]
```

This avoids the overhead of calling `bash -c` and allows for usage of `chdir`, which will be covered later.

### Extra Arguments

When using either the simple command or the full command, you can pass extra arguments to the command by passing them after the command name. For example, you might want to define a command with standard flags that should be used every time, but decide to add an extra flag for a specific invocation.

For example, an extra argument can be passed to the previous command like so:

```bash
❯ ya -x install --force
$ cargo install --path . --force
...
```

The `-x` flag here is just being used to show the command that is being run. It is not required to pass extra arguments.

This feature also allows for parameterized commands, using the relevant logic in the program being run to extract argument values.

e.g. in bash:

```yml
run: |
  echo "The value: '$0' is passed to the command"
```

```bash
❯ ya run "parameterized value"
The value: 'parameterized value' is passed to the command
```

### Interactive Commands

You might also want to save a command that you would like to run interactively. For example, you might want to save a command that starts up a Docker container, and connects you to the shell that starts up in that container. You can do that like so:

```yml
docker_shell:
  prog: docker
  args: ["run", "-it", "--rm"]
  cmd: ubuntu
```

To use this config, you would run a command similar to the previous example:

```bash
❯ ya docker_shell
root@container-id:/#
```

This is equivalent to running the following command:

```bash
❯ docker run -it --rm ubuntu
root@container-id:/#
```

### Chdir

Using the `chdir` key, you can specify a directory to change to before running the command. For example, you might want to always run your tests from the root of your repository. You can do that like so:

```yml
install:
  prog: cargo
  args: ["install", "--path", "."]
  chdir: $GIT_ROOT
```

The value of `chdir` can be any valid path. It can be an absolute path, a relative path, or a path relative to the root of the git repository that the current directory is in.

You can use the special `$GIT_ROOT` variable to refer to the root of the git repository that the current directory is in.

### From

Using the `from` key, you can specify a configuration file to use for a command instead of the current one. For example, you might want to have a central config file that defines standard commands used throughout a monorepo and referenced in subdirectories. You can do that like so:

```yml
# .config/ya.yml
lint:
  prog: cargo
  args: ["clippy", "--all-targets", "--all-features", "--", "-D", "warnings"]
```

```yml
# subdirectory/.config/ya.yml
lint:
  from: $GIT_ROOT/.config/ya.yml
```

```bash
# Within the subdirectory
❯ ya lint
```

By default, `from` will use the name of the calling command as the command to run in the target configuration (in this example, `lint` being used in `subdirectory` results in the `lint` command being called in the root `.config/ya.yml` file). You can override this behavior by specifying a `cmd` key in the command definition.

```yml
# subdirectory/.config/ya.yml
also_lints:
  from: $GIT_ROOT/.config/ya.yml
  cmd: lint
```

### Sub Commands Cont’d

You can also create subcommands of a command. For example, you might have three different ways that you can release, but want to define them all as part of the same command. You can do that like so:

```yml
release:
  sub:
    patch:
      from: $GIT_ROOT/.config/ya/tag.yml
      cmd: release_patch
    minor:
      from: $GIT_ROOT/.config/ya/tag.yml
      cmd: release_minor
    major:
      from: $GIT_ROOT/.config/ya/tag.yml
      cmd: release_major
```

Note that subcommands can be defined as any valid command that `ya` supports, including other subcommands.

### Environment Variables

If you'd like to explicitly list the environment variables that should be passed to a command, you can do so using the `env` key. For example, you might want to pass the `RUST_LOG` environment variable to a command. You can do that like so:

```yml
debug:
  prog: cargo
  args: ["run"]
  env:
    RUST_LOG: debug
```
