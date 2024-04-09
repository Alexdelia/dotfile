local py_workdir="${ZSH_SRC_PATH}tool/python"
local py_src=(
	"pipx.zsh"
	"poetry.zsh"
	"ruff.zsh"
)

local py_file
for py_file in $py_src[@]; do
	if [[ -e "$py_workdir/$py_file" ]]; then
		source "$py_workdir/$py_file"
	else
		printf "$ZSH_WARN\033[1;35m$py_file\033[0m \033[1;33mnot found\033[0m\n"
	fi
done
