{self}: {
  pkgs,
  config,
  lib,
  ...
}: let
  inherit (lib) types;
  inherit (lib.modules) mkIf;
  inherit (lib.options) mkOption mkEnableOption;
  inherit (pkgs.stdenv.hostPlatform) system;
  cfg = config.services.botoh;
in {
  options.services.botoh = {
    enable = mkEnableOption ''
      Enable botoh
    '';

    package = mkOption {
      type = types.package;
      inherit (self.packages.${system}) default;
    };
    log_level = mkOption {
      type = types.str;
      default = "info";
    };
    environmentFiles = mkOption {
      type = types.listOf types.path;
      default = [];
      example = ["/run/twitch_auth"];
      description = ''
        set twitch oauth / id
      '';
    };
  };

  config = mkIf cfg.enable {
    systemd.services.botoh = {
      wantedBy = ["multi-user.target"];
      environment = {
        RUST_LOG = cfg.log_level;
      };
      serviceConfig = {
        EnvironmentFile = cfg.environmentFiles;
        ExecStart = "${cfg.package}/bin/botoh";
      };
    };
  };
}
