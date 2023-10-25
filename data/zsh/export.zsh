if [[ ! -z "$P_WARE" ]]; then
	export PATH="$P_WARE:$PATH"
	export PATH="$P_WARE/utility:$PATH"

	if [[ ! -z "$P_WARE/flutter/bin" ]]; then
		export PATH="$P_WARE/flutter/bin:$PATH"
	fi
fi

export ZSH_COMPDUMP="$ZSH/cache/zcompdump-$HOST-$ZSH_VERSION"

export GIT_EDITOR="vim"

export MAIL="adelille@student.42.fr"

export ANDROID_NDK="$HOME/Android/Sdk/ndk"
