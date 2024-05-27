#! /usr/bin/env -S nix shell nixpkgs#bash nixpkgs#uutils-coreutils-noprefix nixpkgs#git nixpkgs#ripgrep nixpkgs#skim nixpkgs#bat --command bash

IDENTITIES=$(git config --global --name-only --get-regexp "user.*..name" | rg 'user\.(.*)\.name' -or '$1')
ID=$(
	echo "${IDENTITIES}" |
		sk --preview='
			git config --global --get-regexp "user."{}".*" \
			| sort -r \
			| cut -d" " -f2 \
			| bat --color=always -pp -l=qml' \
			--preview-window=down:3
)

if ! git config --global --get-regexp "user.${ID}.name" >/dev/null; then
	exit 78
fi

git config user.name "$(git config user.${ID}.name)"
git config user.email "$(git config user.${ID}.email)"

printf "name:\t%s\nemail:\t%s\n" \
	"$(git config user.name)" \
	"$(git config user.email)" |
	bat --color=always -pp -l=qml
