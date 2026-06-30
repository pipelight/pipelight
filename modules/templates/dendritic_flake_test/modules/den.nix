{
  inputs,
  den,
  normal,
  lib,
  ...
}: {
  imports = [
    inputs.den.flakeModules.default
  ];
  den.hosts.x86_64-linux.default.users.anon = {};

  den.aspects.default = {
    includes = [
      den.batteries.hostname
      den.aspects.anon
    ];
    nixos = {pkgs, ...}: {
      users.users.anon = {
        isNormalUser = true;
        initialPassword = "anon";
      };
    };
  };

  den.aspects.anon = {
    includes = [
      den.batteries.define-user
      den.batteries.primary-user
    ];
    nixos = {user, ...}: {
      imports = [
        inputs.pipelight.nixosModules.pipelight-init
      ];
      services."pipelight-init".enable = true;
    };
    homeManager = {pkgs, ...}: {
      home.packages = [pkgs.vim];
    };
  };
}
