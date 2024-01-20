export FLUTTER_PATH="$HOME/.w/flutter"

[[ -d "$FLUTTER_PATH" ]] &&
	export PATH="$FLUTTER_PATH/bin:$PATH" && export FLUTTER_INSTALLED=1 ||
	printf "$ZSH_WARN\033[1;35mflutter\033[0m \033[1;33mnot found in \033[0m\033[1;35m$FLUTTER_PATH\033[0m\n"
