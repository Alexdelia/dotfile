if [[ ! -z "$P_WARE" ]]; then
	export PATH="$P_WARE:$PATH"
	export PATH="$P_WARE/utility:$PATH"
fi

export ZSH_COMPDUMP="$ZSH/cache/zcompdump-$HOST-$ZSH_VERSION"

export GIT_EDITOR="vim"

export MAIL="adelille@student.42.fr"

export ANDROID_NDK="/home/alex/Android/Sdk/ndk"
