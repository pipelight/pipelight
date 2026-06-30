{
  lib,
  den,
  ...
}: {
  systems = lib.mkDefault lib.systems.flakeExposed;

  den.default.nixos.system.stateVersion = "25.11";
  den.default.homeManager.home.stateVersion = "25.11";

  den.default.includes = [
    den.batteries.inputs'
    den.batteries.self'
  ];
}
