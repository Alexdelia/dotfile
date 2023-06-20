function chrono() {
	if [[ $# -eq 1 ]]; then
		printf $(($(date -u +%s.%N) - $1))
	elif [[ $# -eq 2 ]]; then
		printf $(($2 - $1))
	elif [[ $# -eq 0 ]]; then
		printf $(date -u +%s.%N)
	else
		echo -e "usage: \033[1m$0\033[0m              \treturn current UTC time
       \033[1m$0 \033[35m<start>\033[0m      \treturn elapsed time since \033[1;35m<start>\033[0m
       \033[1m$0 \033[35m<start> <end>\033[0m\treturn elapsed time between \033[1;35m<start>\033[0m and \033[1;35m<end>\033[0m"
		return 1
	fi
	return 0
}

function gimme() {
	if [[ $# -lt 1 ]]; then
		echo -e "usage: \033[1m$0 \033[35m<package0> <package1> <package2> ...\033[0m"
		return 1
	fi

	case "$(uname -n)" in
	"Decim" | "Nona")
		sudo apt update || return 1
		sudo apt install "$@" -y || return 1
		;;
	"Oculus")
		sudo pacman -S "$@" || return 1
		;;
	*)
		echo -e "\033[1m;31munknown host\033[39m: \033[35m$(uname -n)\033[0m"
		return 1
		;;
	esac
}

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
		echo -e "usage: \033[1m$0 \033[34m[OPTION] \033[35m<package0> <package1> <package2> ...\033[0m

option:
	\033[1m-h\033[0m, \033[1m--help\033[0m\tshow this help message and exit
	\033[1m-u\033[0m, \033[1m--no-update\033[0m\tdo not update package list"
		return 1
	fi

	sudo apt update || return 1

	local pkg
	for pkg in "$@"; do
		if ! is_pkg_installed "$pkg"; then
			printf "\033[32minstalling \033[1m%s\033[0m\n" "$pkg"
			sudo apt install "$pkg" -y || return 1
		fi
	done
}

function gen_text() {
	if [[ $# -gt 2 ]]; then
		echo -e "usage: \033[1m$0 \033[35m[len] [line]\033[0m
	default \033[1;35mlen\033[0m = 1000, \033[1;35mline\033[0m = 10000
generate random alphanumeric text with \033[1;35mlen\033[0m length per line and \033[1;35mline\033[0m lines"
		return 1
	fi

	local len=1000
	local line=10000

	if [[ $# -gt 0 ]]; then
		line=$1
	fi
	if [[ $# -gt 1 ]]; then
		len=$2
	fi

	cat /dev/urandom | tr -dc '[:alnum:]' | fold -w "${1:-$len}" | head -n "$line"
}

function rmbk() {
	local CMD='find . -type d \( -name ".git*" -o -path "./target" -o -name "node_modules" \) -prune -o -type f -name "*~"'

	if [[ $# -lt 1 ]]; then
		local start="$(chrono)"

		local n="$(eval $CMD -print -exec rm -f {} + | wc -l)"

		local elapsed="$(chrono "$start")"

		printf "\033[1;32m$n\033[0m in \033[1;32m%.0f\033[0m \033[32mms\033[0m\n" "$((elapsed * 1000))"

		return 0
	fi

	local HELP="usage: \033[1m$0 \033[35m[option]\033[0m
option:
	\033[1m-h\033[0m, \033[1m--help\033[0m\tshow this help message and exit
	\033[1m-p\033[0m, \033[1m--print\033[0m\tprint found files
	\033[1m-s\033[0m, \033[1m--show\033[0m\tdelete found files and print them"

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

function drm() {
	local HELP="usage: \033[1m$0 \033[35m<type1> <type2> ...\033[0m
type:
	\033[1mc\033[0m, \033[1mcontainer\033[0m\tstop and remove all containers
	\033[1mi\033[0m, \033[1mimage\033[0m\tremove all images
	\033[1mv\033[0m, \033[1mvolume\033[0m\tremove all volumes
	\033[1mn\033[0m, \033[1mnetwork\033[0m\tremove all networks
	\033[1ma\033[0m, \033[1mall\033[0m, \033[1my\033[0m, \033[1myes\033[0m\tremove all containers, images, volumes and networks"

	if [[ $# -lt 1 ]]; then
		echo -e $HELP
		return 1
	fi

	function _no_to_remove() {
		echo -e "no \033[1;35m$1\033[0m to \033[1;34m$2\033[0m"
	}

	local d

	local t
	for t in "$@"; do
		case "$t" in
		"c" | "container")
			d="$(docker ps -qa)"
			if [[ -z "$d" ]]; then
				_no_to_remove "container" "remove"
				continue
			fi
			docker stop $d && docker rm $d
			;;
		"i" | "image")
			d="$(docker images -qa)"
			if [[ -z "$d" ]]; then
				_no_to_remove "image" "remove"
				continue
			fi
			docker rmi $d
			;;
		"v" | "volume")
			d="$(docker volume ls -q)"
			if [[ -z "$d" ]]; then
				_no_to_remove "volume" "remove"
				continue
			fi
			docker volume rm $d
			;;
		"n" | "network")
			docker network prune -f
			;;
		"a" | "all" | "y" | "yes")
			d="$(docker ps -qa)"
			if [[ -z "$d" ]]; then
				_no_to_remove "container" "stop"
			else
				docker stop $d
			fi
			docker system prune -af
			;;
		*)
			echo -e "unknown type:\t\033[1;33m$t\033[0m"
			echo -e $HELP
			return 1
			;;
		esac
	done
}
