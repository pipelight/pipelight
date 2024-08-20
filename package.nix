{
  pkgs ? import <nixpkgs> {},
  lib,
  installShellFiles,
  ...
}:
pkgs.rustPlatform.buildRustPackage rec {
  pname = "pipelight";
  version = "0.8.0";
  src = ./.;
  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  # cargoBuildHook = ''
  # buildPhase = ''
  #   cargo build --release
  # '';
  # installPhase = ''
  #   mkdir -p $out/bin
  #   install -t target/release/${pname} $out/bin
  # '';

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
