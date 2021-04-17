# Init

Initializes ya project.

```yaml
USAGE:
    ya init [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <config>    Location of configuration file [default: .config/ya/ya.yml]
    -n, --name <name>        Name of this project
```

This command will create configuration file(s) at the location specified by the value passed to `-c` (by default `.config/ya/ya.yml`).

Eventually, this command will accept a `template` flag to allow for alternate configurations to be used.
