function gy() {
    if [[ $# -ne 1 ]]; then
        echo -e "usage: \033[1m$0 \033[35m<commit_message>\033[0m"
        return 1
    fi
    
    git status --short
    git add --all
    git status --short
    git commit --message "$1"
    git push
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
    
    git commit -m "$commit"
    git push
}