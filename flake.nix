{
  description = "Pipelight - Automation pipelines";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = inputs:
    with inputs;
      flake-utils.lib.eachDefaultSystem (
        system: let
          pkgs = nixpkgs.legacyPackages.${system};
        in rec {
          packages.default = pkgs.callPackage ./default.nix {};
          devShells.default = pkgs.callPackage ./shell.nix {};
        }
      );
}
