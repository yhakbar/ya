# CLI Arguments

The usage printed when running `ya` without any arguments or using the `-h`/`--help` flag should be kept up to date on the [README.md](/README.md) file.

## Print Config

A simple way to determine which config file is being used is to use the `-p`/`--print` flag. This will print the config file to stdout before running any command (or if no command is provided).

```bash
❯ ya -p
---
run: echo "Hey ya!"
---
```

```bash
❯ ya -p run
---
run: echo "Hey ya!"
---

Hey ya!
```

Using the `-p` flag before running a command can be useful for debugging and sharing configurations.

## Extra Arguments

You can also specify extra arguments to pass to the command specified in your configuration. For example, you might want to have a generic `build` command that you can use to build your project, but want to provide additional flags for targeting different platforms. You can do that like so:

```yml
build:
  prog: cargo
  args: ["build", "--release"]
```

```bash
❯ ya build
    Finished release [optimized] target(s) in 0.08s
```

```bash
❯ ya build --target aarch64-apple-darwin
    Finished release [optimized] target(s) in 0.05s
```

Everything that follows the command name will be passed to the command as extra arguments.

## Execution

Setting the `-x`/`--execute` flag will print any command that `ya` executes before executing it. This can be useful for debugging configurations.

```bash
❯ ya -x run
$ bash -c 'echo "Hey ya!"'
Hey ya!
```
