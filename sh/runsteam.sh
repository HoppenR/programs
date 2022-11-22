#!/bin/bash
set -u

if ! pgrep -x "steam" > /dev/null
then
    steam-native -silent &
    notify-send "Starting steam" -i "/usr/share/pixmaps/steam.png"
fi

declare -A gamearray=(
["Bioshock 2"]="run/8850"
["Bioshock 2 Remastered"]="run/409720"
["Bioshock Infinite"]="run/8870"
["Bioshock Remastered"]="run/409710"
["Borderlands"]="run/8980"
["Borderlands 2"]="run/49520"
["Borderlands: The Pre-Sequel"]="run/261640"
["Borderlands 3"]="run/397540"
["Darkest Dungeon"]="run/262060"
["Darksburg"]="run/939100"
["Dota 2"]="run/570"
["Downward"]="run/506900"
["Dreamfall Chapters"]="run/237850"
["Enter the Gungeon"]="run/311690"
["hackmud"]="run/469920"
["Half Life 2"]="run/220"
["Heaven's Vault"]="run/774201"
["Hollow Knight"]="run/367520"
["Iratus: Lord of the Dead"]="run/807120"
["Killing Floor"]="run/1250"
["Killing Floor 2"]="run/232090"
["Library"]="open/games"
["Legion TD 2"]="run/469600"
["PAYDAY 2"]="run/218620"
["Qern - Undying Thoughts"]="run/512790"
["Rocket League"]="run/252950"
["Rogue Bit"]="run/949790"
["Shadow Warrior 2"]="run/324800"
["Shenzhen I/O"]="run/504210"
["Skul: The Hero Slayer"]="run/1147560"
["Slay The Spire"]="run/646570"
["Slime Rancher"]="run/433340"
["Store"]="store"
["Transistor"]="run/237930"
["Underlords"]="run/1046930"
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
