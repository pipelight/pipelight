{
  pkgs,
  ...
}: 
{

  pipelight = pkgs.rustPlatform.buildRustPackage {
    pname = "pipelight";
    version = "0.7.8";
    src = ./.;
    cargoBuildFlags = "-p pipelight";

    cargoLock = {
      lockFile = ./Cargo.lock;
    };

    nativeBuildInputs = [pkgs.pkg-config];
    PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
  };

  utils = pkgs.rustPlatform.buildRustPackage {
    pname = "utilst";
    version = "0.0.1";
    src = ./.;
    cargoBuildFlags = "-p utils";

    cargoLock = {
      lockFile = ./Cargo.lock;
    };

    nativeBuildInputs = [pkgs.pkg-config];
    PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
  };
}
