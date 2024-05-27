#! /usr/bin/env -S nix shell nixpkgs#bash nixpkgs#git --command bash

git config --global --name-only --get-regex "user.*..name"
