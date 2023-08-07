# Installation

## asdf

```bash
asdf plugin add ya https://github.com/yhakbar/asdf-ya.git
asdf install ya latest
```

## rtx

```bash
rtx plugin add ya https://github.com/yhakbar/asdf-ya.git
rtx install ya latest
```

## Homebrew

```bash
brew tap yhakbar/ya
brew install ya
```

## Cargo

```bash
git clone https://github.com/yhakbar/ya.git
cd ya
```

### With `yadayada`

```bash
cargo install --features yadayada --path .
```

### Without `yadayada`

```bash
cargo install --path .
```

## Manual

Go [here](https://github.com/yhakbar/ya/releases/latest) and download the appropriate executable(s) for your platform.

## Shell Completion

I recommend reading [this](/docs/completions.md) for more information on how to install shell completions.
