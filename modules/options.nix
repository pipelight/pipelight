{
  lib,
  config,
  inputs,
  pkgs,
  ...
}:
with lib; let
  moduleName = "pipelight-init";
in {
  ## Options
  options.services.${moduleName} = {
    enable = mkEnableOption "Enable ${moduleName}.";
  };
}
