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

	local commit=""

	for file in "$@"; do
		git add "$file"
		commit+="\`$file\` "
	done

	git commit --message "$commit"
	git push --quiet
}

function rmbk() {
	local CMD='find . -type d \( -name ".git*" -o -path "./target" -o -name "node_modules" \) -prune -o -type f -name "*~"'

	if [[ $# -lt 1 ]]; then
		local start="$(date -u +%s.%N)"

		local n="$(eval $CMD -print -exec rm -f {} + | wc -l)"

		local end="$(date -u +%s.%N)"
		local elapsed="$((end - start))"

		printf "\033[1;32m$n\033[0m in \033[1;32m%.0f\033[0m \033[32mms\033[0m\n" "$((elapsed * 1000))"

		return 0
	fi

	local HELP="usage: \033[1m$0 \033[35m[option]\033[0m
option:
	\033[1m-h, --help\033[0m\tshow this help message and exit
	\033[1m-p, --print\033[0m\tprint found files
	\033[1m-s, --show\033[0m\tdelete found files and print them"

	if [[ $1 == "-h" || $1 == "--help" ]]; then
		echo -e $HELP
		return 0
	elif [[ $1 == "-p" || $1 == "--print" ]]; then
		eval $CMD -print
	elif [[ $1 == "-s" || $1 == "--show" ]]; then
		eval $CMD -print -exec rm -f {} +
	else
		echo -e "unknown option:\t\033[1;33m$1\033[0m"
		echo -e $HELP
		return 1
	fi
}
