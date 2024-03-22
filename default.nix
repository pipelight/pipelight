{pkgs ? import <nixpkgs> {}}:
pkgs.rustPlatform.buildRustPackage rec {
  pname = "pipelight";
  version = "0.7.22";
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
    installShellFiles
    openssl.dev
    pkg-config
    rustc
    cargo
  ];
  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";

  postInstall = with lib; ''
    installShellCompletion --cmd pipelight \
      --bash autocompletions/pipelight.bash \
      --fish autocompletions/pipelight.fish \
      --zsh  autocompletions/_pipelight
  '';
}
