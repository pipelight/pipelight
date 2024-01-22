{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  # Additional tooling
  nativeBuildInputs = with pkgs; [
    openssl.dev
    pkg-config
    rustc
    cargo
    rust-analyzer # LSP Server
    rustfmt # Formatter
    clippy # Linter
  ];
}
