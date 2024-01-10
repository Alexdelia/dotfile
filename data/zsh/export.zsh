if [[ ! -z "$P_WARE" ]]; then
	export PATH="$P_WARE:$PATH"
	export PATH="$P_WARE/utility:$PATH"

	if [[ ! -z "$P_WARE/flutter/bin" ]]; then
		export PATH="$P_WARE/flutter/bin:$PATH"
	fi
fi

export XDG_CONFIG_HOME="$HOME/.config"
export XDG_CACHE_HOME="$HOME/.cache"
export XDG_DATA_HOME="$HOME/.local/share"

export ZSH_COMPDUMP="$ZSH/cache/zcompdump-$HOST-$ZSH_VERSION"
export ZDOTDIR="$XDG_CONFIG_HOME/zsh"

export GIT_EDITOR="vim"

export MAIL="adelille@student.42.fr"

export ANDROID_NDK="$HOME/Android/Sdk/ndk"

export WAKATIME_HOME="$XDG_CONFIG_HOME/wakatime"
