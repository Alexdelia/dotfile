{
  config,
  pkgs,
  ...
}: {
  home.username = "alex";
  home.homeDirectory = "/home/alex";
  home.stateVersion = "23.11";

  programs.home-manager.enable = true;

  imports = [
    ./pkg/bat.nix
  ];

  home.packages = with pkgs; [
    # neofetch
    uutils-coreutils
  ];
}
