{pkgs ? import <nixpkgs> {}}:
pkgs.rustPlatform.buildRustPackage rec {
  pname = "pipelight";
  version = "0.7.10";
  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };
  cargoBuildHook = ''
    git checkout v0.7.10
    cargo build --release
  '';
  # disable tests
  checkType = "debug";
  doCheck = false;

  nativeBuildInputs = with pkgs; [
    pkg-config
    rustc
    cargo
  ];

  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
}
