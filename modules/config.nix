{
  config,
  pkgs,
  lib,
  inputs,
  ...
}: let
  moduleName = "pipelight-init";
  cfg = config.services.${moduleName};
in
  with lib;
    mkIf cfg.enable {
      systemd.services.pipelight-init_clean_logs = {
        enable = true;
        description = "Clean pipelight-init old logs";
        before = ["pipelight-init_net_pre"];
        # Starts only if mountpoint detected
        unitConfig = {
          ConditionPathExists = "/pipelight-init";
        };
        serviceConfig = with pkgs; let
          package = inputs.pipelight.packages.${system}.default;
        in {
          Type = "oneshot";
          User = "root";
          Group = "users";
          Environment = "PATH=/run/current-system/sw/bin";
          ExecStart = "-${package}/bin/pipelight logs rm";
          WorkingDirectory = "/pipelight-init";
          StandardInput = "null";
          StandardOutput = "journal+console";
          StandardError = "journal+console";
        };
      };

      systemd.services.pipelight-init_net_pre = {
        enable = true;
        description = "Run pipelight as a cloud-init replacement";
        before = ["network.target"];
        wantedBy = ["multi-user.target"];
        # Starts only if mountpoint detected
        unitConfig = {
          ConditionPathExists = "/pipelight-init";
        };
        serviceConfig = with pkgs; let
          package = inputs.pipelight.packages.${system}.default;
        in {
          Type = "oneshot";
          User = "root";
          Group = "users";
          Environment = "PATH=/run/current-system/sw/bin";
          ExecStart = ''
            ${package}/bin/pipelight run init_net_pre --attach -vvv
          '';
          WorkingDirectory = "/pipelight-init";
          StandardInput = "null";
          StandardOutput = "journal+console";
          StandardError = "journal+console";
        };
      };
      systemd.services.pipelight-init_net_post = {
        enable = true;
        description = "Run pipelight as a cloud-init replacement.";
        after = ["network.target"];
        wantedBy = ["multi-user.target"];
        # Starts only if mountpoint detected
        unitConfig = {
          ConditionPathExists = "/pipelight-init";
        };
        serviceConfig = with pkgs; let
          package = inputs.pipelight.packages.${system}.default;
        in {
          Type = "oneshot";
          User = "root";
          Group = "users";
          Environment = "PATH=/run/current-system/sw/bin";
          ExecStart = ''
            ${package}/bin/pipelight run init_net_post --attach -vvv
          '';
          WorkingDirectory = "/pipelight-init";
          StandardInput = "null";
          StandardOutput = "journal+console";
          StandardError = "journal+console";
        };
      };
    }
