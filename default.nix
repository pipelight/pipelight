{pkgs ? import <nixpkgs> {}}:
pkgs.rustPlatform.buildRustPackage rec {
  pname = "pipelight";
  version = "0.7.9";
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
  ];

  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
}
