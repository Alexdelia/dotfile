alias ga="ga" # override git plugin alias
function ga() {
	if [[ $# -eq 0 ]]; then
		git add --all
	else
		git add "$@"
	fi
	git status --short
	echo -e "\n\033[3mun-add:\033[0m\t\033[1;38;2;232;77;49mgit\033[39m restore \033[2m--staged\033[0m \033[1;35m<path>\033[0m\n\033[3m    or:\033[0m\t\033[1;38;2;232;77;49mg\033[39mr\033[2ms \033[35m[path]\033[0m"
}

alias grs="grs" # override git plugin alias
function grs() {
	if [[ $# -eq 0 ]]; then
		git restore --staged .
	else
		git restore --staged "$@"
	fi
	git status --short
}

function gy() {
	if [[ $# -ne 1 ]]; then
		echo -e "usage: \033[1m$0 \033[35m<commit_message>\033[0m"
		return 1
	fi

	git status --short
	git add --all
	git status --short
	git commit --message "$1"
	git push --quiet
}

function gpn() {
	if [[ $# -lt 1 ]]; then
		echo -e "usage: \033[1m$0 \033[35m<file1> <file2> <...>\033[0m"
		return 1
	fi

	commit=""

	for file in "$@"; do
		git add "$file"
		commit+="\`$file\` "
	done

	git commit --message "$commit"
	git push --quiet
}

# function rmbk() {
# 	if [[ $# -lt 1 ]]; then
# 		find . -type d \( -name '.git*' -o -name 'node_modules'\) -prune -o -name '*~' -delete
# 		echo -e "\e[32;1mdone\e[0m"
# 	else
# 		HELP="\033[1m$0 \033[35m[option]\033[0m\n\noption:\n\t\033[1m-h, --help\033[0m\tshow this help message and exit"
# 		CMD='find . -type d \( -name ".git*" \) -prune -o -name "*~"'

# 		if [[ $1 == "-h" || $1 == "--help" ]]; then
# 			echo -e $HELP
# 			return 0
# 		elif [[ $1 == "-p" || $1 == "--print" ]]; then
# 			eval $CMD -print
# 		elif [[ $1 == "-s" || $1 == "--show" ]]; then
# 			eval $CMD -delete -print
# 		else
# 			echo -e "unknown option:\t\033[1;33m$1\033[0m"
# 			echo -e $HELP
# 			return 1
# 		fi
# 	fi
# }
