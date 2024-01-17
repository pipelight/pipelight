# Contributing

## Build and Run

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

## Internal documentation

The main documentaion can be found at [pipelight.dev](https://pipelight.dev/).
It explains the tool usage and functionning, however it doesn't say much on
internal structure.

1. The `INTERNALS.md` gives a quick overlook of how things works.

2. There is a `README.md` detailling what is inside most of modules(crate).

```sh
.
├── cast
│   ├── public
│   ├── README.md
│   └── src
├── cli
│   ├── build.rs
│   └── src
├── exec
│   ├── README.md
│   └── src
├── pipelight
│   ├── README.md
│   └── src
├── switch
│   └── src
├── templates
│   ├── README.md
│   ├── src
│   └── static
├── utils
│   ├── public
│   ├── README.md
│   └── src
└── workflow
    ├── README.md
    └── src
```

3. Each internal modules and functions have exhaustive and improving
   descriptions

## Easy set up on Nixos

Deprecated usage of `nix-shell` command with shell.nix file in favor of
`nix develop` with flake.nix.

To spawn a developing env with lsp formatter and linter.

```sh
nix develop
```

or do it automatically when you `cd` into the directory with
[nix-direnv](https://github.com/nix-community/nix-direnv)
