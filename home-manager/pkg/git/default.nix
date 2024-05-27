{
  config,
  lib,
  pkgs,
  ...
}:
# let
#   gitIdentity =
#     pkgs.writeShellScriptBin "git-identity" (builtins.readFile ./git-identity.sh);
# in
{
  home.packages = with pkgs; [
    # gitIdentity
    (symlinkJoin {
      name = "git-identity";
      paths = [./git-identity.sh];
      buildInputs = [makeWrapper];
      postBuild = ''
        wrapProgram $out/bin/my-script.sh --prefix PATH : ${stdenv.shell}
      '';
    })
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
    };
  };
}
