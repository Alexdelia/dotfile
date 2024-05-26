#! /usr/bin/env -S nix shell nixpkgs#bash nixpkgs#git nixpkgs#uutils-coreutils-noprefix nixpkgs#diffutils nixpkgs#alejandra nixpkgs#ripgrep --command bash

log_folder="./.log/nix/"
mkdir -p $log_folder

format_log="${log_folder}format.log"
alejandra -q . &>"$format_log" || printf "  \033[31m* format\033[0m -> \033[35m%s\033[0m\n" "$format_log"

git add '*.nix'

if git diff --staged --exit-code; then
	printf "  \033[36m* no changes\033[0m\n"
	# exit 0
fi

if command -v home-manager &>/dev/null; then
	home_manager_cmd="home-manager"
else
	printf "  \033[33m* \033[1;35mhome-manager\033[0m \033[33mnot found\033[0m, using \033[1;35mnix run\033[0m\n"
	home_manager_cmd="nix run home-manager#home-manager --"
fi

pkg_before_log="${log_folder}packages-before.log"
pkg_after_log="${log_folder}packages-after.log"
pkg_diff_log="${log_folder}packages-diff.log"

$home_manager_cmd packages | sort >"$pkg_before_log"

home_manager_log="${log_folder}build.log"

$home_manager_cmd switch --flake ".#$USER" &>"$home_manager_log" ||
	printf "  \033[31m* build\033[0m -> \033[35m%s\033[0m\n" "$home_manager_log" &&
	rg -i "warning:\s+(.*)" "$home_manager_log" -or '$1' \
		-N --colors="match:style:nobold" --colors="match:fg:yellow" --color=always | uniq &&
	rg -i "error:\s+(.*)" "$home_manager_log" -or '$1' \
		-N --colors="match:style:nobold" --colors="match:fg:red" --color=always | uniq &&
	exit 1

$home_manager_cmd packages | sort >"$pkg_after_log"

diff "$pkg_before_log" "$pkg_after_log" \
	--unchanged-group-format="" --new-line-format="+ %L" --old-line-format="- %L" |
	sort -r -k 1,1 \
		>"$pkg_diff_log" || exit 1

generation_id=$(home-manager generations | head -1 | cut -d' ' -f5)
msg="home-manager generation id $generation_id"

if [ -s "$pkg_diff_log" ]; then
	msg="$msg\n\npackage changes:\n$(cat "$pkg_diff_log")"
fi

git commit -m "$msg"
