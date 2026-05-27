default:
    just --list

alias pal := publish-android-local
alias pag := publish-android-github
alias paa := publish-android-central
alias ps := publish-swift

build:
    cd bbqr-swift && just build

gen:
    cd bbqr-swift && just gen

publish-android-local:
    cd bbqr-android && just publish-local

publish-android-github:
    cd bbqr-android && just publish-github

publish-android-central:
    cd bbqr-android && just publish-central

publish-swift version:
    git tag v{{version}}
    git push origin v{{version}}
