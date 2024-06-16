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
      serviceConfig = {
        EnvironmentFile = cfg.environmentFiles;
        ExecStart = "${cfg.package}/bin/botoh";
      };
    };
  };
}
