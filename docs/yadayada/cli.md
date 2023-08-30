# YadaYada

The `yadayada` binary is a sister binary to `ya` that provides utilities for local development that compliment it.

## Installation

To install `yadayada`, follow the instructions found in [the installation section](../install.md).

## Usage

```bash
❯ yadayada
yadayada - save yourself some chatter

Usage: yadayada [OPTIONS] [COMMAND]

Commands:
  install   Install command completion for `ya` and `yadayada`
  keys      Print keys of a config
  alias     Alias a command, and add to config
  template  Manage templates
  help      Print this message or the help of the given subcommand(s)

Options:
      --no-color  No color
  -h, --help      Print help
  -V, --version   Print version
```

## Commands

The commands present in `yadayada` assist in improving developer experience.

### Install

The `install` command is used to install shell completion for `ya` and `yadayada`. The best supported shell is `fish`, with best effort support a handful of other shells.

### Keys

The `keys` command is used to print the keys of a config file. This is used to provide a reliable way to get the keys of a config file for shell completion of `ya`. It can also give you a quick way to test if you are picking up the `ya` config you expect, and the commands available.

### Alias

The `alias` command is used to alias a command, and add it to the `ya` config. This is useful for quickly storing a task that you perform often, and want to be able to easily replicate with `ya`.

### Template

The `template` command is used to simply manage templates. This is useful for quickly creating and using template(s) from the command line.

```bash
❯ yadayada template
Manage templates

Usage: yadayada template [COMMAND]

Commands:
  list   List templates
  stamp  Stamp a template from source to target
  save   Save a template from an existing file path
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

#### List

The `list` command is used to list templates.

```bash
❯ yadayada template list --help
List templates

Usage: yadayada template list [OPTIONS]

Options:
  -d, --dir <DIR>  The templates directory to list
  -h, --help       Print help
```

`yadayada` uses [handlebars](https://handlebarsjs.com/) template directories. The default templates directory location is `./templates`. To manually specify a different directory, use the `-d` flag.

#### Save

The `save` command is used to save a file/directory template from an existing file path. If a file is specified, the file will be saved as the only template in a template directory named after the file. If a directory is specified, all (non-hidden by default) directories within the directory will be saved as templates in a template directory named after the directory. All templates within the directory will be saved with a suffix of `.hbs`.

```bash
❯ yadayada template save --help
Save a template from an existing file path

Usage: yadayada template save [OPTIONS] <FILE>

Arguments:
  <FILE>  The file path of the template

Options:
  -d, --dir <DIR>                The templates directory to save to
  -p, --parameters <PARAMETERS>  Strings to replace with parameters in the template
  -H, --hidden                   Save hidden files too
  -h, --help                     Print help
```

Using the `-p` flag, you can specify strings to replace with parameters in the template. This is useful for creating templates that are parameterized.

For example, if you have a template that contains the string `Pikachu`, you can use the `-p` flag to replace that string like so `pokemon=Pikachu`. Any templates that are saved that contain the string `Pikachu` will then be replaced with the handlebar expression `{{pokemon}}` when they are saved.

```bash
❯ echo 'Pikachu is a pokemon!' > facts.txt

❯ yadayada template save -p 'pokemon=Pikachu' facts.txt
✔ No templates directory found. Would you like to create one? · yes
Creating template "facts.txt"

❯ cat templates/facts.txt/facts.txt.hbs
{{pokemon}} is a pokemon!
❯ cat templates/facts.txt/.config/yadayada.yml
template:
  inputs:
  - pokemon
```

#### Stamp

The `stamp` command is used to stamp a template. All file templates located within a template directory will be stamped into the target directory with the suffix `.hbs` removed.

```bash
❯ yadayada template stamp --help
Stamp a saved template

Usage: yadayada template stamp [OPTIONS] <SOURCE> <TARGET>

Arguments:
  <SOURCE>  The template to stamp
  <TARGET>  The target of the template

Options:
  -d, --dir <DIR>                The templates directory to fetch from
  -p, --parameters <PARAMETERS>  Strings to replace parameters with from the template
  -h, --help                     Print help
```

Use the `-p` flag to specify strings to replace parameters with from the template. This is useful for stamping templates that are parameterized.

For example, if you have a template that contains the handlebar expression `{{pokemon}}`, you can use the `-p` flag to replace that expression like so `pokemon=Charizard`. Any templates that are stamped that contain the handlebar expression `{{pokemon}}` will then be replaced with the string `Charizard` when they are stamped.

```bash
❯ yadayada template stamp -p 'pokemon=Charizard' facts.txt .
❯ cat facts.txt
Charizard is a pokemon!
```
