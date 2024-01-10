if ((! $+commands[bun])); then
	return
fi

export BUN_INSTALL="$P_TOOL/bun"
export PATH="$BUN_INSTALL/bin:$PATH"

# bun completions
[ -s "${BUN_INSTALL}/_bun" ] && source "${BUN_INSTALL}/_bun"
