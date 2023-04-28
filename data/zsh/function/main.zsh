local src=(
	"git.zsh"
	"sh.zsh"
	"w2x.zsh"
)

local file
for file in $src[@]; do
	# in ZSH_SRC_PATH ./function/$file
	if [[ -e "${ZSH_SRC_PATH}function/$file" ]]; then
		source "${ZSH_SRC_PATH}function/$file"
	else
		printf "$ZSH_WARN\033[1;35m$file\033[0m \033[1;33mnot found\033[0m\n"
	fi
done
