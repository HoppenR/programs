#!/bin/bash
set -u

if ! pgrep -x "steam" > /dev/null
then
	steam-native -silent &
	notify-send "Starting steam" -i "/usr/share/pixmaps/steam.png"
fi

declare -A gamearray=(
["Bioshock 2"]="run/8850"
["Bioshock Infinite"]="run/8870"
["Bioshock: Remastered"]="run/409710"
["Borderlands"]="run/8980"
["Borderlands 2"]="run/49520"
["Borderlands: The Pre-Sequel"]="run/261640"
["Darkest Dungeon"]="run/262060"
["Dota 2"]="run/570"
["Downward"]="run/506900"
["Dreamfall Chapters"]="run/237850"
["Enter the Gungeon"]="run/311690"
["hackmud"]="run/469920"
["Half Life 2"]="run/220"
["Killing Floor"]="run/1250"
["Library"]="open/games"
["PAYDAY 2"]="run/218620"
["Qern - Undying Thoughts"]="run/512790"
["Rocket League"]="run/252950"
["Rogue Bit"]="run/949790"
["Shenzhen I/O"]="run/504210"
["Slay The Spire"]="run/646570"
["Slime Rancher"]="run/433340"
["Store"]="store"
["Transistor"]="run/237930"
["Valley"]="run/378610"
["Victor Vran"]="run/345180"
)

gamelist=$(printf "%s\\n" "${!gamearray[@]}" | sort)

if ! game="$(dmenu -i -l ${#gamearray[@]} -fn 'Liberation Mono:size=10' \
	-nb '#272B35' -nf '#F5F7FA' -sf '#F5F7FA' -sb '#5675B9' -p '»' \
	<<< "$gamelist")"
then
	exit 1
fi

if ! grep -qx "$game" <<< "$gamelist"
then
	game="$(grep -im1 "$game" <<< "$gamelist")" || exit 2
fi

if [[ -z $game ]]
then
	exit 3
fi

steam-native steam://"${gamearray["$game"]}"
