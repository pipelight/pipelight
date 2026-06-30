{
  lib,
  inputs,
  ...
}: {
  flake-file.inputs = {
    rust-overlay.url = "github:oxalica/rust-overlay";
  };
  systems = lib.mkDefault lib.systems.flakeExposed;
  perSystem = {
    pkgs,
    system,
    ...
  }: {
    _module.args = import inputs.nixpkgs {
      inherit system;
      overlays = [inputs.rust-overlay.overlays.default];
    };
    packages.default = pkgs.rustPlatform.buildRustPackage rec {
      pname = "pipelight";
      version = (builtins.fromTOML (lib.readFile ../pipelight/Cargo.toml)).package.version;
      src = ../.;

      cargoLock = {
        lockFile = ../Cargo.lock;
      };

      # disable tests
      checkType = "debug";
      doCheck = false;

      nativeBuildInputs = with pkgs; [
        pkg-config
        installShellFiles
      ];

      buildInputs = with pkgs;
        lib.optionals stdenv.isDarwin (with darwin.apple_sdk.frameworks; [
          CoreFoundation
          CoreServices
          IOKit
          Security
        ]);

      postInstall = ''
        installShellCompletion --cmd ${pname} \
          --bash ./autocompletion/${pname}.bash \
          --fish ./autocompletion/${pname}.fish \
          --zsh  ./autocompletion/_${pname}
      '';
    };
  };
}
