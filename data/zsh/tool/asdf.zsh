export ASDF_DATA_DIR="$P_TOOL/asdf"
export ASDF_DIR="$ASDF_DATA_DIR"
export ASDF_CONFIG_FILE="$ASDF_DATA_DIR/asdfrc"

if [[ -d "$HOME/.asdf" ]]; then
	printf "$ZSH_INFO\033[1;36mmoving \033[1;35masdf \033[1;36mfrom \033[1;33m~/.asdf \033[1;36mto \033[1;31m$ASDF_DATA_DIR\033[0m\n"

	mv "$HOME/.asdf" "$ASDF_DATA_DIR"
fi

if [[ ! -f "$ASDF_CONFIG_FILE" ]]; then
	printf "legacy_version_file = yes\n" >"$ASDF_CONFIG_FILE"
fi
