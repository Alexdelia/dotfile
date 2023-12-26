unalias ga # remove git plugin alias
function ga() {
	if [[ $# -eq 0 ]]; then
		git add --all
	else
		git add "$@"
	fi
	git status --short
	echo -e "\n\033[3mun-add:\033[0m\t\033[1;38;2;232;77;49mgit\033[39m restore \033[2m--staged\033[0m \033[1;35m<path>\033[0m\n\033[3m    or:\033[0m\t\033[1;38;2;232;77;49mg\033[39mr\033[2ms \033[35m[path]\033[0m"
}

unalias grs # remove git plugin alias
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

function gt() {
	if [[ $# -ne 1 ]]; then
		echo -e "usage: \033[1m$0 \033[35m<commit_message>\033[0m"
		return 1
	fi

	git status --short
	git add --all
	git status --short
	git commit --message "$1"
}

function gpn() {
	if [[ $# -lt 1 ]]; then
		echo -e "usage: \033[1m$0 \033[35m<file1> <file2> <...>\033[0m"
		return 1
	fi

	local commit=""

	local file
	for file in "$@"; do
		git add "$file"
		commit+="\`$file\` "
	done

	git commit --message "$commit"
	git push --quiet
}

function gmc() {
	if [[ $# -lt 1 ]]; then
		echo -e "usage: \033[1m$0 \033[35m<branch1> <branch2> <...>\033[0m"
		return 1
	fi

	local base_branch="$(git branch --show-current)"

	local current="$base_branch"
	local b
	for b in "$@"; do
		echo -e "\033[1;35m$b\033[0m"
		git checkout "$b" --quiet || return 1
		git pull --quiet || return 1
		git merge "$current" --quiet || return 1
		git push --quiet || return 1
		current="$b"
	done

	git checkout "$base_branch" --quiet || return 1
	git merge "$current" --quiet || return 1
	git push --quiet || return 1
}

function gmcf() {
	if [[ $# -lt 2 ]]; then
		echo -e "usage: \033[1m$0 \033[35m<from_branch> <to_branch1> <to_branch2> <...>\033[0m"
		return 1
	fi

	local original_branch="$(git branch --show-current)"

	git checkout "$1" --quiet || return 1
	git pull --quiet || return 1

	gmc "${@:2}" || return 1

	git checkout "$original_branch" --quiet || return 1
}
