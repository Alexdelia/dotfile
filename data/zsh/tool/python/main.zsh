local workdir="${ZSH_SRC_PATH}tool/python"
local src=(
	"pipx.zsh"
	"poetry.zsh"
	"ruff.zsh"
)

local file
for file in $src[@]; do
	if [[ -e "$workdir/$file" ]]; then
		source "$workdir/$file"
	else
		printf "$ZSH_WARN\033[1;35m$file\033[0m \033[1;33mnot found\033[0m\n"
	fi
done
