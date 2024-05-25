{
  description = "Alexdelia's nix config";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-23.11";
    # nixpkgs-unstable.url = "github:nixos/nixpkgs/nixpkgs-unstable";

    home-manager = {
      url = "github:nix-community/home-manager/release-23.11";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,

    home-manager,
    ...
  }: let
    system = "aarch64-linux";
    # system = "x86_64-linux";

    pkgs = nixpkgs.legacyPackages.${system};
    # pkgs-unstable = nixpkgs-unstable.legacyPackages.${system};
  in {
    homeConfigurations = {
      alex = home-manager.lib.homeManagerConfiguration {
        inherit pkgs;
        modules = [
          ./home.nix
        ];
      };
    };
  };
}
