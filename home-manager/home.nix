{
  config,
  pkgs,
  ...
}: {
  home.username = "alex";
  home.homeDirectory = "/home/alex";
  home.stateVersion = "23.11";

  programs.home-manager.enable = true;
  targets.genericLinux.enable = true;

  imports = [
    ./pkg/git
    ./pkg/bat.nix
    ./pkg/eza.nix
  ];

  home.packages = with pkgs; [
    uutils-coreutils
    ripgrep

    # neofetch

    ## nix
    alejandra

    ## docker
    oxker
  ];
}
