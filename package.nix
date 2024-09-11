{
  pkgs ? import <nixpkgs> {},
  lib,
  installShellFiles,
  ...
}:
pkgs.rustPlatform.buildRustPackage rec {
  pname = "pipelight";
  version = "0.8.0";
  src = (builtins.fromTOML (lib.readFile ./pipelight/Cargo.toml)).package.version;
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
