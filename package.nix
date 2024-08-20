{
  pkgs ? import <nixpkgs> {},
  lib,
  ...
}:
pkgs.rustPlatform.buildRustPackage rec {
  pname = "pipelight";
  version = "0.8.0";
  src = ./.;
  cargoLock = {
    lockFile = ./Cargo.lock;
  };
  # disable tests
  checkType = "debug";
  doCheck = false;

  nativeBuildInputs = with pkgs; [
    installShellFiles
    openssl.dev
    pkg-config
    rustc
    cargo
  ];
  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";

  postInstall = ''
    installShellCompletion --bash ./autocompletion/${pname}.bash
    installShellCompletion --fish ./autocompletion/${pname}.fish
    installShellCompletion --zsh  ./autocompletion/_${pname}
  '';
}
