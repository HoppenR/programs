#!/bin/bash
set -u

NAME=$(basename "$0")
OPTIONS=-p
LONGOPTIONS=push
if ! PARSED=$(getopt -o "$OPTIONS" -l "$LONGOPTIONS" --name "$NAME" -- "$@")
then
	exit 2
fi

eval set -- "$PARSED"

while true
do
	case "$1" in
		"-p"|"--push")
			GitPush=true
			shift
			;;
		--)
			shift
			break
			;;
		*)
			echo >&2 "$NAME: unrecognized non-option argument '$1'"
			exit 3
			;;
	esac
done

if [[ -n ${1:-} ]]
then
	echo >&2 "$NAME: error parsing arguments"
	exit 3
fi

declare -a repositories=(config-files/ notes/ programs/ .vim/pack/default/start/vim-polyglot/
.vim/pack/default/start/YouCompleteMe/ .vim/pack/default/start/nerdtree)

cd ~ || exit 8
for D in "${repositories[@]}"
do
	pushd "$D" > /dev/null || exit 8
		printf -v line "%${#D}s"
		echo -e "$D\\n${line// /─}"
		if [[ "${GitPush:-false}" == true ]]
		then
			if ! git log --oneline --exit-code origin/master..HEAD
			then
				git push
			fi
		else
			git pull --rebase
		fi
	popd > /dev/null || exit 8
done
