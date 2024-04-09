if (($+commands[poetry])); then
	return
fi

if ((! $+commands[pipx])); then
	printf "$ZSH_ERR\033[1;31mcannot install \033[1;35mpoetry \033[1;31mwithout \033[1;35mpipx\033[0m\n"

	return
fi

printf "$ZSH_INFO\033[1;32minstalling \033[1;35mpoetry\033[0m\n"

pipx install poetry

mkdir $ZSH_CUSTOM/plugins/poetry || return
poetry completions zsh >$ZSH_CUSTOM/plugins/poetry/_poetry

export POETRY_INSTALLED=1
