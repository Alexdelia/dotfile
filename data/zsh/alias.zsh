# alias	norminette="~/.norminette/norminette.rb"
# alias	norminette_v2="~/.norminette_old/norminette.rb"
alias	nor="norminette"
alias	py42="~/goinfre/$USER/miniconda3"

# alias	gcc="clang-9"
# alias	clang="clang-9"
alias	clangW="clang -Wall -Werror -Wextra"

# not an alias, but whatever
function gy() {
    if [[ -z "$1" ]]; then
        echo -e "usage: \033[1m$0 \033[35m<commit_message>\033[0m"
        return 1
    fi
    
    git status --short
    git add --all
    git status --short
    git commit --message "$1"
    git push
}

alias	ga="git add --all && git status --short"
alias 	gc="git commit --message"
alias	gst="git status --short"
alias	gtree="git log --graph --oneline --decorate"
alias	gpn="/home/alex/p/bin/utility/gpn.sh"

alias	sl="ls"
alias	mk="make"
alias	amke="make"
alias	mkae="make"
alias	maek="make"
alias	ivm="vim"
alias	imv="vim"
alias	gti="git"

alias	c="cargo"
alias	s="sudo"

alias	up="sudo apt update && sudo apt upgrade -y && sudo apt autoremove -y"
alias	dlist="echo -e '\n\t\033[32;1mcontainer\033[0m' && \
docker ps -a && \
echo -e '\n\t\033[32;1mimage\033[0m' && \
docker images -a && \
echo -e '\n\t\033[32;1mnetwork\033[0m' && \
docker network ls && \
echo -e '\n\t\033[32;1mvolume\033[0m' && \
docker volume ls && \
echo"

alias	neo="neofetch"
alias	to="bashtop"
alias	poke="pokemon-colorscripts -r"

alias	rmbk=~/p/bin/utility/rmbk.sh
# alias	w2x=/var/lib/flatpak/app/com.github.nihui.waifu2x-ncnn-vulkan/current/8bf9ef4885a0ca7426344fd1a080f8290c42373bb3e7aa5327a189c9319b4a27/export/bin/com.github.nihui.waifu2x-ncnn-vulkan
alias	waifu2x=/home/alex/Ware/waifu2x-ncnn-vulkan-20210521-ubuntu/waifu2x-ncnn-vulkan
alias	w2x=/home/alex/Ware/waifu2x-ncnn-vulkan-20210521-ubuntu/waifu2x-ncnn-vulkan
alias	auto_w2x=/home/alex/p/bin/utility/auto_waifu2x.sh
alias	aw2x=/home/alex/p/bin/utility/auto_waifu2x.sh
alias	ani-cli=/home/alex/Ware/ani-cli/ani-cli
alias	yuzu=/home/alex/Games/Yuzu/yuzu-20220305-ed691f09c.AppImage