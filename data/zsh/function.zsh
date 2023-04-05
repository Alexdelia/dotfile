alias	ga="ga"	# override git plugin alias
function ga() {
    if [[ $# -eq 0 ]]; then
        git add --all
    else
        git add "$@"
    fi
    git status --short
    echo -e "\n\033[3mun-add:\033[0m\t\033[1;38;2;232;77;49mgit\033[39m restore \033[2m--staged\033[0m \033[1;35m<path>\033[0m\n\033[3m    or:\033[0m\t\033[1;38;2;232;77;49mg\033[39mr\033[2ms \033[35m[path]\033[0m"
}

alias	grs="grs"	# override git plugin alias
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

function gpn() {
    if [[ $# -lt 1 ]]; then
        echo -e "usage: \033[1m$0 \033[35m<file1> <file2> <...>\033[0m"
        return 1
    fi
    
    commit=""
    
    for file in "$@"
    do
        git add "$file"
        commit+="\`$file\` "
    done
    
    git commit --message "$commit"
    git push --quiet
}