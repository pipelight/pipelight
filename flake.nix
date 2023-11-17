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
        in rec {
          packages.default = let
            name = "pipelight";
            package = pkgs.rustPlatform.buildRustPackage {
              pname = name;
              version = "0.7.8";
              src = ./.;

              cargoLock = {
                lockFile = ./Cargo.lock;
              };
              cargoBuildHook = ''
                cargo build --release
              '';
              # disable tests
              checkType = "debug";
              doCheck = false;

              nativeBuildInputs = with pkgs; [
                pkg-config
                rustc
                cargo
                rustfmt
                rust-analyzer
                clippy
              ];

              PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
            };
          in
            package;
          default = packages.pipelight;
        }
      );
}
