# Contributing

## Setup

Download the source code.

```sh
git clone <repo_url>
```

And build on your machine.

```sh
cd <repo_name>
cargo build
```

Run tests with

```sh
cd <repo_name>
cargo tests
```

Try the built binary.

```sh
cargo run --bin pipelight
```

```sh
cargo run --bin pipelight --help
```

```sh
cargo run --bin pipelight ls
```

## On Nixos

Deprecated usage of `nix-shell` command with shell.nix file in favor of
`nix develop` with flake.nix.

To spawn a developing env with lsp formatter and linter.

```sh
nix develop
```

or do it automatically when you `cd` into the directory with
[nix-direnv](https://github.com/nix-community/nix-direnv)
