function is_pkg_installed() {
	if [[ $# -lt 1 ]]; then
		echo -e "usage: \033[1m$0 \033[35m<package>\033[0m"
		return 1
	fi

	dpkg -s "$1" &>/dev/null
	return $?
}

function install_pkg() {
	if [[ $# -lt 1 ]]; then
		echo -e "usage: \033[1m$0 \033[35m<package0> <package1> <package2> ...\033[0m"
		return 1
	fi

	s apt update || return 1

	local pkg
	for pkg in "$@"; do
		if ! is_pkg_installed "$pkg"; then
			printf "\033[32minstalling \033[1m%s\033[0m\n" "$pkg"
			s apt install "$pkg" -y || return 1
		fi
	done
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
