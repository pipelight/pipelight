{
  pkgs ? import <nixpkgs> {},
  lib,
  ...
}:
with lib;
with pkgs;
  rustPlatform.buildRustPackage rec {
    pname = "pipelight";
    src = ./.;
    version = "0.7.20";

    cargoLock = {
      lockFile = ./Cargo.lock;
    };

    # disable tests
    checkType = "debug";
    doCheck = false;

    nativeBuildInputs = with pkgs; [
      openssl.dev
      pkg-config
      rustc
      cargo
    ];
    PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
  }
