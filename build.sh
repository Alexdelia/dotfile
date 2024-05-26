#! /usr/bin/env -S nix shell nixpkgs#bash nixpkgs#git nixpkgs#uutils-coreutils nixpkgs#alejandra nixpkgs#ripgrep --command bash

log_folder="./.log/nix/"
uutils-mkdir -p $log_folder

format_log="${log_folder}format.log"
alejandra -q . &>"$format_log" || uutils-printf "  \033[31m* format\033[0m -> \033[35m${format_log}\033[0m\n"

if command -v home-manager &>/dev/null; then
	home_manager_cmd="home-manager"
else
	uutils-printf "  \033[33m* \033[1;35mhome-manager\033[0m \033[33mnot found\033[0m, using \033[1;35mnix run\033[0m\n"
	home_manager_cmd="nix run home-manager#home-manager --"
fi

pkg_before_log="${log_folder}packages-before.log"
pkg_after_log="${log_folder}packages-after.log"

$home_manager_cmd packages | uutils-sort >"$pkg_before_log"

home_manager_log="${log_folder}build.log"

$home_manager_cmd switch --flake ".#$USER" &>"$home_manager_log" ||
	uutils-printf "  \033[31m* build\033[0m -> \033[35m${home_manager_log}\033[0m\n" &&
	rg -i "warning:\s+(.*)" "$home_manager_log" -or '$1' -N --colors="match:style:nobold" --colors="match:fg:yellow" &&
	rg -i "error:\s+(.*)" "$home_manager_log" -or '$1' -N --colors="match:style:nobold" --colors="match:fg:red" &&
	exit 1

$home_manager_cmd packages | uutils-sort >"$pkg_after_log"
