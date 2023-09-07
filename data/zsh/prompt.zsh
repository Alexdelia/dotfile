PROMPT="%(?:%{$fg_bold[green]%}➜:%{$fg_bold[red]%}➜)  %(!:🫚 :)%{$fg_bold[magenta]%}%c%{$reset_color%}"
PROMPT+=' $(git_prompt_info)'

ZSH_THEME_GIT_PROMPT_PREFIX="%{$fg_bold[blue]%}git:(%{$fg[red]%}"
ZSH_THEME_GIT_PROMPT_SUFFIX="%{$reset_color%} "
ZSH_THEME_GIT_PROMPT_DIRTY="%{$fg[blue]%}) %{$fg[yellow]%}✗"
ZSH_THEME_GIT_PROMPT_CLEAN="%{$fg[blue]%})"

bindkey '^H' backward-kill-word
bindkey '5~' kill-word

function chrono() {
	if [[ $# -eq 1 ]]; then
		printf $(($(date -u +%s.%N) - $1))
	elif [[ $# -eq 2 ]]; then
		printf $(($2 - $1))
	elif [[ $# -eq 0 ]]; then
		printf $(date -u %s)
	else
		echo -e "usage: \033[1m$0\033[0m              \treturn current UTC time
		\033[1m$0 \033[35m<start>\033[0m      \treturn elapsed time since \033[1;35m<start>\033[0m
		\033[1m$0 \033[35m<start> <end>\033[0m\treturn elapsed time between \033[1;35m<start>\033[0m and \033[1;35m<end>\033[0m"
		return 1
	fi
	return 0
}

function pretty_elapsed() {
	if [[ $# -ne 1 ]]; then
		echo -e "usage: \033[1m$0 \033[35m<elapsed>\033[0m"
		return 1
	fi

	local ela="$1"
	ela="$(($ela * 1000))"

	if [[ $ela -lt 100 ]]; then
		return 0
	fi

	local PRE_N="\033[0m\033[1;38;2;255;183;227m"
	local PRE_UNIT="\033[0m\033[38;2;107;77;96m"
	local POST="\033[0m\n"

	if [[ $ela -lt 1000 ]]; then
		printf "$PRE_N%.0f${PRE_UNIT}ms$POST" "$ela"
	elif [[ $ela -lt 4000 ]]; then
		printf "$PRE_N%.3f${PRE_UNIT}s$POST" "$((ela / 1000))"
	elif [[ $ela -lt 60000 ]]; then
		printf "$PRE_N%.0f${PRE_UNIT}s$POST" "$((ela / 1000))"
	else
		printf "$PRE_N%.0f${PRE_UNIT}m ${PRE_N}%.0f${PRE_UNIT}s$POST" "$((ela / 60000))" "$((ela % 60000 / 1000))"
	fi
	return 0
}

function preexec() {
	__ZSH_EXEC_START="$(chrono)"
}

function precmd() {
	if [ $__ZSH_EXEC_START ]; then
		local elapsed="$(chrono "$__ZSH_EXEC_START")"

		printf "\n"
		pretty_elapsed "$elapsed"

		# unset __ZSH_EXEC_START
	fi
}
