if [[ ! -z "$P_WARE" ]]; then
	export PATH="$P_WARE:$PATH"
	export PATH="$P_WARE/utility:$PATH"
fi

export GIT_EDITOR="vim"

export ANDROID_NDK="/home/alex/Android/Sdk/ndk"
