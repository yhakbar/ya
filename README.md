# Ya

A very simple automation tool for folks that don't want to learn `make`.

## Usage

```bash
❯ ya
ya - yet another command runner

Usage: ya [OPTIONS] [COMMAND] [EXTRA_ARGS]...

Arguments:
  [COMMAND]        The command to run
  [EXTRA_ARGS]...  The extra arguments to pass to the command

Options:
  -q, --quiet            Suppress extra output
  -c, --config <CONFIG>  The config file
  -p, --print            Print the config file before running
  -x, --execution        Print the executed command before executing it
      --no-color         No color
  -h, --help             Print help
  -V, --version          Print version
```

`ya` is a tool that automates tasks that you specify using a yaml config file.

## Installation

See the following for installation instructions: [docs/install.md](docs/install.md).

## Examples

See the following for some example config files: [examples](examples).

## Config

Read the following for information on the config file: [docs/config.md](docs/config.md).

## CLI

Read the following for information on the CLI: [docs/cli.md](docs/cli.md).

## Completions

Read the following for information on the completions: [docs/completions.md](docs/completions.md).

## Yadayada

Read the following for information on the developer tool, [yadayada](/docs/yadayada/cli.md).

