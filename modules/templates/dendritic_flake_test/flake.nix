{
  description = "A dendritic flake that uses normal.nixos";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    flake-file.url = "github:vic/flake-file";
    import-tree.url = "github:denful/import-tree";
    den.url = "github:denful/den";
    home-manager = {
      url = "github:nix-community/home-manager";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    pipeligh.url = "github:crocuda/pipelight.nixos?ref=dev";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake {
      inherit inputs;
    } (inputs.import-tree ./modules);
}
