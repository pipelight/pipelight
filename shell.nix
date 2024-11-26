{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  buildInputs = with pkgs.buildPackages; [
    pkg-config
    rust-bin.stable.latest.default
    pkgs.rust-analyzer
  ];
}
