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
    ./pkg/eza.nix
  ];

  home.packages = with pkgs; [
    uutils-coreutils
    ripgrep

    # neofetch

    ## nix
    alejandra
  ];
}
