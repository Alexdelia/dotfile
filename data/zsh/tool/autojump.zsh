if [[ -s "$XDG_CONFIG_HOME/autojump/share/autojump/autojump.zsh" ]]; then
	export PATH="$XDG_CONFIG_HOME/autojump/bin:$PATH"
	export FPATH="$XDG_CONFIG_HOME/autojump/functions:$FPATH"

	source "$XDG_CONFIG_HOME/autojump/share/autojump/autojump.zsh"
	return
fi

printf "$ZSH_INFO\033[1;35mautojump \033[1;36mneed action\033[0m\n"

# check if installed in ~/.autojump
if [[ -d "$HOME/.autojump" ]]; then
	printf "$ZSH_INFO\033[1;35mautojump \033[1;36mfound in \033[1;33m~/.autojump\033[0m, \033[1;36mremoving it\033[0m\n"

	rm -rf "$HOME/.autojump"
fi

# install autojump
printf "$ZSH_INFO\033[1;32minstalling \033[1;35mautojump\033[0m\n"

local tmp_repo="/tmp/autojump_repo"

rm -rf "$tmp_repo"

git clone -q "https://github.com/wting/autojump.git" "$tmp_repo" && cd $tmp_repo &&
	./install.py --destdir "$XDG_CONFIG_HOME/autojump" && cd - &&
	printf "$ZSH_INFO\033[1;35mautojump \033[1;32minstalled\033[0m, \033[1;36myou only need to reload \033[1;35mzsh\033[0m\n" ||
	printf "$ZSH_ERR\033[1;35mautojump \033[1;31minstall failed\033[0m\n"
