{
  lib,
  inputs,
  ...
}: {
  flake-file.inputs = {
    rust-overlay.url = "github:oxalica/rust-overlay";
  };
  systems = lib.mkDefault lib.systems.flakeExposed;
  perSystem = {
    pkgs,
    system,
    ...
  }: {
    # _module.args = import inputs.nixpkgs {
    #   inherit system;
    #   overlays = [inputs.rust-overlay.overlays.default];
    # };
    devShells.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        pkg-config
        (rust-bin.stable.latest.default.override {
          targets = (builtins.fromTOML (lib.readFile ../rust-toolchain.toml)).toolchain.targets;
        })
        wasm-pack
        wasm-bindgen-cli
        rust-analyzer
      ];
    };
  };
}
