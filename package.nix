{
  pkgs ? import <nixpkgs> {},
  lib,
  installShellFiles,
  ...
}:
pkgs.rustPlatform.buildRustPackage rec {
  pname = "pipelight";
  version = (builtins.fromTOML (lib.readFile ./pipelight/Cargo.toml)).package.version;
  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  # disable tests
  checkType = "debug";
  doCheck = false;

  nativeBuildInputs = with pkgs; [
    pkg-config
    installShellFiles
  ];

  buildInputs = with pkgs;
    [
      openssl
    ]
    ++ lib.optionals stdenv.isDarwin (with darwin.apple_sdk.frameworks; [
      CoreFoundation
      CoreServices
      IOKit
      Security
    ]);

  postInstall = ''
    installShellCompletion --bash ./autocompletion/${pname}.bash
    installShellCompletion --fish ./autocompletion/${pname}.fish
    installShellCompletion --zsh ./autocompletion/_${pname}
  '';
}
