{pkgs, ...}: let
  packages = let
    rust_crates = [
  ## Dependencies small utilities crates
  "utils"
  "exec"
  "cast"
  "templates"
  ## Core crates with interdependencies
  "workflow"
  "cli"
  "switch"
  ## Bin package
  "pipelight"
    ];
    # Return package definition
    make_package = name:
      pkgs.rustPlatform.buildRustPackage {
        pname = name;
        version = "0.7.8";
        src = ./.;
        cargoBuildFlags = "-p ${name}";

        cargoLock = {
          lockFile = ./Cargo.lock;
        };
        nativeBuildInputs = [pkgs.pkg-config];
        PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
      };
  in
    builtins.listToAttrs (
      builtins.map (
        u: {
          name = u;
          value = make_package u;
        }
      )
      rust_crates
    );
in {
  pipelight = packages.pipelight;
  utils = packages.utils;
}
