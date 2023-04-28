function __check_set_path() {
	if [ -z "$P_WARE" ]; then
		echo -e "\033[1;31mP_WARE\033[0m \033[31mis not set\033[0m"
		return 1
	fi

	if [ -z "$P_WAIFU2X" ]; then
		echo -e "\033[1;31mP_WAIFU2X\033[0m \033[31mis not set\033[0m
\033[1;31mP_W2X\033[0m define the path to the \033[1;33mnunif repository\033[0m"
		return 1
	fi
}

function w2x() {
	__check_set_path || return 1

	local HELP="usage: \033[1m$0 \033[35m[option]\033[0m"
	local noise=2
	local scale=2
	local file=()

	local arg
	for arg in "$@"; do
		# check if start with -
		if [[ $arg =~ ^- ]]; then
			# check -nX or --noiseX (X == 0 | 1 | 2 | 3)
			if [[ $arg =~ ^(-n|--noise)([0-3])$ ]]; then
				noise="${BASH_REMATCH[2]}"
			# check -sX or --scaleX (X == 0 | 2 | 4)
			elif [[ $arg =~ ^(-s|--scale)([024])$ ]]; then
				scale="${BASH_REMATCH[2]}"
			else
				echo -e "unknown option:\t\033[1;33m$arg\033[0m"
				echo -e $HELP
				return 1
			fi
		else
			file+=("$arg")
		fi
	done

	local param=("--method")
	if [[ $noise -gt 0 && $scale -eq 4 ]]; then
		param+="noise_scale4x"
	elif [[ $noise -gt 0 && $scale -eq 2 ]]; then
		param+="noise_scale"
	elif [[ $noise -gt 0 ]]; then
		param+="noise"
	elif [[ $scale -eq 4 ]]; then
		param+="scale4x"
	elif [[ $scale -eq 2 ]]; then
		param+="scale"
	else
		echo -e "\033[1;33mnothing to do\033[0m"
		echo -e $HELP
		return 1
	fi

	if [[ $noise -gt 0 ]]; then
		param+="--noise-level"
		param+="$noise"
	fi

	pwd=$(pwd)
	cd "$P_WAIFU2X"

	source ".venv/bin/activate"
	local f
	for f in "${file[@]}"; do
		if [ ! -f "$pwd/$f" ]; then
			echo -e "$ZSH_WARN\033[1;35m$pwd/$f\033[0m \033[1;33mnot found\033[0m"
			continue
		fi

		local out="$pwd/${f%.*}_w2x.png"

		echo -e "\033[32mprocessing \033[1m$f\033[0m"
		python -m waifu2x.cli $param -i "$pwd/$f" -o "$out"
		file "${out%\/*}" | awk '{print $1,"\033[1;31m",$5,$6,$7,"\033[0m\n"}' | tr ',' ' '
	done
	deactivate

	cd "$pwd"
}

function w2x_install() {
	__check_set_path || return 1

	local PYTHON_V="3.10"

	install_pkg "git" "python$PYTHON_V" "python$PYTHON_V-venv" "git-core" "libmagickwand-dev" "libsnappy-dev" "libraqm-dev" || return 1

	# check if cloned
	if [ ! -d "$P_WAIFU2X" ]; then
		printf "\033[32mcloning \033[1mnunif\033[0m\n"
		git clone https://github.com/nagadomi/nunif.git "$P_WAIFU2X" || return 1
	else
		printf "\033[32mupdating \033[1mnunif\033[0m\n"
		git -C "$P_WAIFU2X" pull || return 1
	fi

	# check if venv exists
	if [ ! -d "$P_WAIFU2X/.venv" ]; then
		printf "\033[32mcreating \033[1mvenv\033[0m\n"
		python$PYTHON_V -m venv "$P_WAIFU2X/.venv"
	fi

	source "$P_WAIFU2X/.venv/bin/activate"

	printf "\033[32minstalling \033[1mpip dependencies\033[0m\n"
	pip3 install torch torchvision torchaudio torchtext
	pip3 install -r "$P_WAIFU2X/requirements.txt"

	local pwd=$(pwd)
	cd "$P_WAIFU2X"
	printf "\033[32mdownloading \033[1mmodels\033[0m\n"
	python -m waifu2x.download_models
	cd "$pwd"

	deactivate
}
