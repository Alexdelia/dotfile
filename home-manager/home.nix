{ config, pkgs, ... }:

{
  home.username = "alex";
  home.homeDirectory = "/home/alex";
  home.stateVersion = "23.11";
  home.packages = with pkgs; [
    neofetch
  ];
  programs.home-manager.enable = true;
}
