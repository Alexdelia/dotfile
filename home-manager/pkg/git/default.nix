{
  config,
  lib,
  pkgs,
  ...
}: let
  gitIdentity =
    pkgs.writeScriptBin "git-identity" (builtins.readFile ./git-identity.sh);
in {
  home.packages = with pkgs; [
    gitIdentity
  ];

  programs.git = {
    enable = true;

    extraConfig = {
      user.useConfigOnly = true;

      user.self.name = "Alexdelia";
      user.self.email = "alexandre.delille.57@gmail.com";

      user.school.name = "adelille";
      user.school.email = "adelille@student.42.fr";

      user.work.name = "Alexandre Delille";
      user.work.email = "alexandre@terros.io";

      push.autoSetupRemote = true;
    };

    aliases = {
      identity = "! git-identity";
      id = "! git-identity";

      vommit = "commit";
    };
  };
}
