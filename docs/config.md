# Config

## Ya Configurations

Ya configurations are yaml files that provide instructions on automation of up to three classes of activities:

- [builds](./build.md)
- [runs](./run.md)
- [shells](./shell.md)

By defining a config for one of those like so:

```yaml
config:
  run:
    plugin: shell
    config:
      command: echo hello
```

You allow `ya` to use that configuration for automation of a task.

```bash
$ ya run
hello
```

By default, `ya` will look for a file located at [.config/ya/ya.yml](/.config/ya/ya.yml).

You can select an alternate configuration file, either by specifying the configuration by path:

```bash
$ ya build -c .config/ya/compile.yml
...
   Compiling clap v2.33.3
   Compiling structopt v0.3.21
   Compiling serde_yaml v0.8.17
   Compiling handlebars v3.5.4
   Compiling ya v0.1.0 (/Users/yhakbar/repos/src/github.com/yhakbar/ya)
    Finished release [optimized] target(s) in 15.00s
```

If the file is located within the default configuration directory, you can refer to the filename without its extension like so:

```bash
$ ya build -c compile
...
   Compiling clap v2.33.3
   Compiling structopt v0.3.21
   Compiling serde_yaml v0.8.17
   Compiling handlebars v3.5.4
   Compiling ya v0.1.0 (/Users/yhakbar/repos/src/github.com/yhakbar/ya)
    Finished release [optimized] target(s) in 15.00s
```

## Command

The config command currently just prints a given configuration file. I plan to have this command function more like a config parsing command one day (e.g. `ya config name` or `ya config .config.build.command`).

```bash
$ ya config

---
name: ya
config:
  build:
    plugin: shell
    config:
      shell: ~
      command: "docker build -t ya-builder -f .config/docker/Dockerfile .\n"
      argument_replacement_key: ~
  run:
    plugin: shell
    config:
      shell: ~
      command: "if [[ \"$(docker images -q ya-builder 2> /dev/null)\" == \"\" ]]; then\n  ya build\nfi\n\ndocker run -t --rm -v $PWD:/app ya-builder -c \"$@\"\n"
      argument_replacement_key: ~
  shell:
    plugin: shell
    config:
      shell: ~
      command: "if [[ \"$(docker images -q ya-builder 2> /dev/null)\" == \"\" ]]; then\n  ya build\nfi\ndocker run -it --rm -v $PWD:/app ya-builder\n"
deps:
  - name: Dockerfile
    src: default_docker
    file: "../docker/Dockerfile"

```
