# Build

Runs a build according to ya [configuration](./config).

```yaml
USAGE:
    ya build [FLAGS] [OPTIONS] [arguments]...

FLAGS:
    -h, --help            Prints help information
    -n, --no-arguments    Don't parse arguments e.g. $@
    -V, --version         Prints version information

OPTIONS:
    -c, --config <config>    Location of configuration file [default: .config/ya/ya.yml]

ARGS:
    <arguments>...    Optional arguments to pass into command
```

The build command automates action(s) taken to interact with your project.

## Ya Config Syntax

```yaml
config:
  build:
    plugin: shell
    config:
      command: |
        mkdir -p .ya-cache
        echo "fake" | tee .ya-cache/fake-artifact
```

```bash
$ ya build
fake
```

### Arguments

Pass arguments from the command line like this:

```yaml
config:
  build:
    plugin: shell
    config:
      command:  |
        mkdir -p .ya-cache
        echo "fake $@" | tee .ya-cache/fake-artifact
```

This allows you to add arguments to your run command:

```bash
$ ya run file
fake file
```

### Replacement Key

If you don't want `$@` to be the string that captures input from the user, you can change it like so:

```yaml
config:
  build:
    plugin: shell
    config:
      command: |
        mkdir -p .ya-cache
        echo "fake <file_text>" | tee .ya-cache/fake-artifact
      argument_replacement_key: <file_text>
```

```bash
$ ya run this file
fake this file
```

### This Project's Configurations

This project uses two ya configs to orchestrate different builds.

#### Main

```yaml
config:
  build:
    plugin: shell
    config:
      command: |
        docker build -t ya-builder -f .config/docker/Dockerfile 
```

This is the ya build config that is located [here](/.config/ya/ya.yml). It builds a Docker image based on [this Dockerfile](/.config/docker/Dockerfile). It's meant to be used in conjunction with the `run` and `shell` commands of this configuration, to ensure that usage of this project does not require installing any dependencies directly.

#### Compilation

```yaml
config:
  build:
    plugin: shell
    config:
      command: |
        if [[ "$(docker images -q ya-builder 2> /dev/null)" == "" ]]; then
          docker build -t ya-builder -f .config/docker/Dockerfile .config/docker
        fi

        cargo clean

        ya run "cargo build --release --target aarch64-unknown-linux-gnu"

        cargo build --release --target x86_64-apple-darwin
        cargo build --release --target aarch64-apple-darwin

        mkdir -p .ya-cache
        for build in $(fd -tf 'ya$' --exclude deps --exclude debug target); do
          cache_name="$(echo $build | sd 'target/([^/]+)/release/ya' 'ya-$1')"
          cp "$build" .ya-cache/"$cache_name"
        done
```

This build configuration (located [here](/.config/ya/compile.yml)) builds the `ya` binaries uploaded to GitHub as part of a release.
