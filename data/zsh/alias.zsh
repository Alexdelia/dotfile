# alias	norminette="~/.norminette/norminette.rb"
# alias	norminette_v2="~/.norminette_old/norminette.rb"
alias nor="norminette"

# alias	gcc="clang-9"
# alias	clang="clang-9"
alias clangW="clang -Wall -Werror -Wextra"
alias cw="clang -Wall -Werror -Wextra"

# here for now
git config --global push.autoSetupRemote true

alias gc="git commit --message"
alias gst="git status --short"
alias gtree="git log --graph --oneline --decorate"

alias ll="ls -lah"

alias sl="ls"
alias nj="just"
alias mk="make"
alias amke="make"
alias mkae="make"
alias maek="make"
alias ivm="vim"
alias vmi="vim"
alias gti="git"

alias s="sudo"
alias c="cargo"
alias po="poetry"
alias pr="poetry run"

alias up="sudo apt update && sudo apt upgrade -y && sudo apt autoremove -y"

alias dlist="echo -e '\n\t\033[32;1mcontainer\033[0m' && \
docker ps -a && \
echo -e '\n\t\033[32;1mimage\033[0m' && \
docker images -a && \
echo -e '\n\t\033[32;1mnetwork\033[0m' && \
docker network ls && \
echo -e '\n\t\033[32;1mvolume\033[0m' && \
docker volume ls && \
echo"
alias dcd="docker compose -f docker-compose.dev.yml"

alias neo="neofetch"
alias poke="pokemon-colorscripts -r"
alias cg="java -jar $P_CG_LOCAL"
