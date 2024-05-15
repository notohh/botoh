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
  };

  config = mkIf cfg.enable {
    systemd.services.botoh = {
      wantedBy = ["multi-user.target"];
      serviceConfig.ExecStart = "${cfg.package}/bin/botoh";
    };
  };
}
