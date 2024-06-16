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
    twitch_id = mkOption {
      type = types.str;
    };
    twitch_oauth = mkOption {
      type = types.str;
    };
  };

  config = mkIf cfg.enable {
    systemd.services.botoh = {
      wantedBy = ["multi-user.target"];
      serviceConfig.ExecStart = "${cfg.package}/bin/botoh";
      environment = {
        TWITCH_ID = cfg.twitch_id;
        TWITCH_OAUTH = cfg.twitch_oauth;
      };
    };
  };
}
