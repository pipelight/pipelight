{pkgs ? import <nixpkgs> {}}:
pkgs.rustPlatform.buildRustPackage rec {
  pname = "pipelight";
  version = "0.7.20";
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
    openssl.dev
    pkg-config
    rustc
    cargo
  ];
  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
}
