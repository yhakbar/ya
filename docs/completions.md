# Completions

`ya` supports completion for multiple shells, and can be installed in a couple of different ways.

The best supported shell is `fish`, with best effort support a handful of other shells.

Command completion is best done when used with the sister binary in this project, `yadayada`, which provides utilities for local development.

## Installation

Assuming you've installed `yadayada`, you can use the `install` command to install shell completion for any shell it supports (only `fish` as of now).

### Fish Install

The following is what you need to run to install shell completions for `fish`:

```fish
yadayada i
```

This will install the completions for `ya` and `yadayada` into the appropriate directory expected by `fish`.

### Manual Installation

All shell completions are created during the build process and are available as part of the release artifacts.

You will need to use the appropriate completion file for your shell, and place it in the appropriate directory.

Unfortunately, this can be quite variable, so I recommend reading the documentation for your shell to find out where to place the completion file.

## Shell Completion

The default shell completion that can be expected is full completion for `yadayada` and switch completion for `ya`.

### Default Completion

```fish
❯ yadayada <tab>
help  (Print this message or the help of the given subcommand(s))  install  (Install command completion for `ya` and `yadayada`)  keys  (Print keys of a config)
```

```fish
❯ yadayada -<tab>
-h  --help  (Print help)  -V  --version  (Print version)  --no-color  (No color)
```

```fish
❯ ya -<tab>
-c  --config  (The config file)  -p  --print  (Print the config file before running)  -V  --version                                     (Print version)  --no-color            (No color)
-h  --help         (Print help)  -q  --quiet                (Suppress excess output)  -x  --execution  (Print the executed command before executing it)  --sd  (Search and displacements)
```

The reason `ya` does not always have full completion for commands is because it is not possible to know at compilation time what the available commands are going to be at runtime, due to the nature of the `ya` config system.

### Fish Completion

Because I use `fish` as my shell, I have made an effort to provide the best completion for it. As a consequence, `ya` has full completion for commands by inspecting the configuration file at runtime.

```fish
❯ ya <tab>
build   install  release_major  release_patch
format  lint     release_minor  test
```
