{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    pkg-config
    (rust-bin.stable.latest.default.override {
      targets = (builtins.fromTOML (lib.readFile ./rust-toolchain.toml)).toolchain.targets;
    })
    pkgs.wasm-pack
    pkgs.wasm-bindgen-cli
  ];
}
