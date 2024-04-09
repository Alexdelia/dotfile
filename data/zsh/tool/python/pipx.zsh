if (($+commands[pipx])); then
	return
fi

printf "$ZSH_INFO\033[1;32minstalling \033[1;35mpipx\033[0m\n"

python3 -m pip install --user pipx
python3 -m pipx ensurepath
