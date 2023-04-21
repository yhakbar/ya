# Ya YAML Spec

## Spec

```yml
command_as_string: String

command_as_mapping:
  prog: String
  args: Optional List of Strings
  cmd: Optional String

  pre_cmds: Optional List of Strings
  post_cmds: Optional List of Strings

  pre_msg: Optional String
  post_msg: Optional String

  chdir: Optional String
```

- `command_as_string` is a string that will be run as a command. This is equivalent to `command_as_mapping` with `prog` set to `bash`, `args` set to `-c` and `cmd` set to the value.
- `command_as_mapping` is a mapping defining multiple configurations for a command.
  - `prog` is the program to run.
  - `args` is a list of arguments to pass to the program.
  - `cmd` is the command to run.
  - `pre_cmds` is a list of commands found in the config to run before the command.
  - `post_cmds` is a list of commands found in the config to run after the command.
  - `pre_msg` is a message to print before running the command.
  - `post_msg` is a message to print after running the command.
  - `chdir` is the directory to change to before running the command.

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
- `$HOME/ya.yml`
- `$HOME/ya.yaml`
- `$HOME/.config/ya.yml`
- `$HOME/.config/ya.yaml`

Where `$GIT_ROOT` is the root of the git repository that the current directory is in, and `$HOME` is the home directory of the current user.

Note that although you the highest precedence config file is the one in the current directory, I recommend tucking your config file into the `.config` directory for repository configurations. This reduces clutter in your configurations and allows you to quickly override the config file by creating a temporary config in the current directory.

You can also specify a config file to use explicitly with the `-c`/`--config` flag.

## Building Your Own Config

Ya does not come with any default commands. You must build your own config file to use it. The config file is a YAML file that contains a mapping of commands to run. The keys are the names of commands and the values are the commands to run.

Example config files can be found in the [examples](/examples) directory.

### Single Commands

The simplest config file would look something like this:

```yml
run: echo "Hey ya!"
```

It's a single key called `run` with a value of `echo "Hey ya!"`. By default, `ya` will run the command `bash -c` followed by the value of a key in the config file. In this case, the following invocation of `ya`:

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
root@495b1451aeb5:/#
```

This is equivalent to running the following command:

```bash
❯ docker run -it --rm ubuntu
root@495b1451aeb5:/#
```

### Pre and Post Commands

You can also specify commands to run before and/or after a given command. For example, you might want to run your linter before you run your tests. You can do that like so:

```yml
lint:
  prog: cargo
  args: ["clippy", "--all-targets", "--all-features", "--", "-D", "warnings"]

test:
  prog: cargo
  args: ["test", "--all-targets", "--all-features"]
  pre_cmds:
    - lint
```

Note that if any command exits with a non-zero exit code, the entire command will exit with a non-zero exit code and exit early. In this example, if your linter fails, your tests will not run.

### Pre and Post Messages

You can also specify messages to print before and/or after the command you specify. For example, you might want to print a message before you run your tests. You can do that like so:

```yml
test_difficult_stuff:
  cmd: "echo Testing difficult stuff..."
  pre_msg: ">>> Running tests..."
  post_msg: ">>> Tests finished!"
```

```bash
❯ ya test_difficult_stuff
>>> Running tests...
Testing difficult stuff...
>>> Tests finished!
```

Note that you can use the `-q`/`quiet` flag to suppress the output of `pre_msg` and `post_msg`:

```bash
❯ ya -q test_difficult_stuff
Testing difficult stuff...
```

### Chdir

Using the `chdir` key, you can specify a directory to change to before running the command. For example, you might want to always run your tests from the root of your repository. You can do that like so:

```yml
install:
  prog: cargo
  args: ["install", "--path", "."]
  chdir: $GIT_ROOT
```

You can also use a `chdir` key that starts with the `$HOME` variable to change to a path relative to your home directory, or use local a relative path.
