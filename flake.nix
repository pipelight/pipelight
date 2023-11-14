{
  description = "A devShell example";

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
          code = pkgs.callPackage ./. {inherit nixpkgs system rust-overlay;};
        in rec {
          packages = {
            pipelight = code.pipelight;
            # utils = code.utils;
            default = packages.pipelight;
          };
        }
      );
}
