if (( $+commands[ruff])); then
	return
fi

if ((! $+commands[pipx])); then
	printf "$ZSH_ERR\033[1;31mcannot install \033[1;35mruff \033[1;31mwithout \033[1;35mpipx\033[0m\n"

	return
fi

printf "$ZSH_INFO\033[1;32minstalling \033[1;35mruff\033[0m\n"

pipx install poetry
