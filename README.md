# Ya

A very simple automation tool for lazy folks that don't want to learn `make`.

## Usage

```bash
❯ ya
Automation tool for lazy people

Usage: ya [OPTIONS] [COMMAND] [EXTRA_ARGS]...

Arguments:
  [COMMAND]        The command in the config to use
  [EXTRA_ARGS]...  The extra arguments to pass to the command

Options:
  -q, --quiet            Suppress the output of `pre_msg` and `post_msg`
  -c, --config <CONFIG>  The config file to use
  -p, --print            Print the config file before running the command
      --sd <SD>          Search and displacements to make in the command before running it. Expects a key and value separated by an `=`. e.g. `--sd key=value`
  -h, --help             Print help
  -V, --version          Print version
```

`ya` is a tool that automates tasks that you specify using a yaml config file.

## Installation

See the following for installation instructions: [docs/install.md](docs/install.md).

## Examples

See the following for some example config files: [examples](examples).

## Config

Read the following for more information on the config file: [docs/config.md](docs/config.md).

## CLI

Read the following for more information on the CLI: [docs/cli.md](docs/cli.md).
