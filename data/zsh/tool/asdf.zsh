export ASDF_DATA_DIR="$P_TOOL/asdf"
export ASDF_DIR="$ASDF_DATA_DIR"
export ASDF_CONFIG_FILE="$ASDF_DATA_DIR/asdfrc"

if [[ -d "$HOME/.asdf" ]]; then
	printf "$ZSH_INFO\033[1;36mmoving \033[1;35masdf \033[1;36mfrom \033[1;33m~/.asdf \033[1;36mto \033[1;31m$ASDF_DATA_DIR\033[0m\n"

	mv "$HOME/.asdf" "$ASDF_DATA_DIR"

	asdf reshim
fi

if [[ ! -d "$ASDF_DATA_DIR" ]]; then
	printf "$ZSH_INFO\033[1;32minstalling \033[1;35masdf\033[0m\n"

	git clone https://github.com/asdf-vm/asdf.git "$ASDF_DATA_DIR" &&
		cd "$ASDF_DATA_DIR" &&
		git checkout "$(git describe --abbrev=0 --tags)" &&
		cd - &&
		printf "$ZSH_INFO\033[1;35masdf \033[1;32minstalled\033[0m\n" ||
		printf "$ZSH_ERR\033[1;35masdf \033[1;31minstall failed\033[0m\n"
fi

if [[ ! -f "$ASDF_CONFIG_FILE" ]]; then
	printf "legacy_version_file = yes\n" >"$ASDF_CONFIG_FILE"
fi
