{...}: {
  flake.nixosModules."pipelight-init" = {
    config,
    lib,
    self,
    pkgs,
    ...
  }:
    with lib; let
      system = pkgs.stdenv.hostPlatform.system;
      package = self.packages.${system}.default;
    in {
      ## Options
      options.services."pipelight-init" = {
        enable = mkEnableOption "Enable pipelight-init.";
      };

      config = mkIf config.services."pipelight-init".enable {
        systemd.services.pipelight-init_clean_logs = {
          enable = true;
          description = "Clean pipelight-init old logs";
          before = ["pipelight-init_net_pre.service"];
          # Starts only if mountpoint detected
          unitConfig = {
            ConditionPathExists = "/pipelight-init";
          };
          serviceConfig = {
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
          serviceConfig = {
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
          serviceConfig = {
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
      };
    };
}
