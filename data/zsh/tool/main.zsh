export P_TOOL="$HOME/.w"

mkdir -p "$P_TOOL"

local workdir="${ZSH_SRC_PATH}tool"
local src=(
	"autojump.zsh"
	"flutter.zsh"
	"asdf.zsh"
	"bun.zsh"
)

local file
for file in $src[@]; do
	if [[ -e "$workdir/$file" ]]; then
		source "$workdir/$file"
	else
		printf "$ZSH_WARN\033[1;35m$file\033[0m \033[1;33mnot found\033[0m\n"
	fi
done
