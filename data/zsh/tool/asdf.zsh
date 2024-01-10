# if asdf is still in ~/.asdf, print a warning
if [[ -d "$HOME/.asdf" ]]; then
	printf "$ZSH_ERROR\033[1;35masdf\033[0m \033[1;33mstill in \033[1;31m~/.asdf\033[0m\n"
fi

export ASDF_DATA_DIR="$P_TOOL/asdf"
export ASDF_DIR="$ASDF_DATA_DIR"
