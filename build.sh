#! /usr/bin/env -S nix shell nixpkgs#bash nixpkgs#git nixpkgs#uutils-coreutils-noprefix nixpkgs#diffutils nixpkgs#alejandra nixpkgs#ripgrep nixpkgs#ncurses --command bash

log_folder="./.log/nix/"
mkdir -p $log_folder

format_log="${log_folder}format.log"
alejandra -q . &>"$format_log" || printf "  \033[31m* format\033[0m -> \033[35m%s\033[0m\n" "$format_log"

git add '*.nix'

if git --paginate diff --staged --exit-code; then
	printf "  \033[36m* no changes\033[0m\n"
	exit 0
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

$home_manager_cmd packages | sort >"$pkg_before_log" || exit 1

home_manager_log="${log_folder}build.log"

function process_log_line() {
	while IFS= read -r line; do
		# print with max size tput cols
		printf "\r\033[K%.*s" "$(tput cols)" "$line"
		printf "%s\n" "$line" >>"$home_manager_log"
	done
}

true >"$home_manager_log"

if ! $home_manager_cmd switch --flake ".#$USER" 2>&1 | process_log_line; then
	printf "  \033[31m* build\033[0m -> \033[35m%s\033[0m\n" "$home_manager_log"
	rg -i "warning:\s+(.*)" "$home_manager_log" -or '$1' \
		-N --colors="match:style:nobold" --colors="match:fg:yellow" --color=always | uniq
	rg -i "error:\s+(.*)" "$home_manager_log" -or '$1' \
		-N --colors="match:style:nobold" --colors="match:fg:red" --color=always | uniq

	exit 78
fi

printf "\r\033[K  \033[32m* build\033[0m\n"

$home_manager_cmd packages | sort >"$pkg_after_log" || exit 1

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
