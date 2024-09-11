{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  buildInputs = with pkgs.buildPackages; [
    openssl
    pkg-config
    rust-bin.nightly.latest.default
    pkgs.rust-analyzer
  ];
}
