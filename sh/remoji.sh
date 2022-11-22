#!/bin/bash
set -u

db=$(cat emoji-db.txt)

dmenuconfig=(-i -fn 'Liberation Mono:size=10' -nb '#272B35' -nf '#F5F7FA' -sf '#F5F7FA' -sb '#5675B9' -p '»')

if ! match=$(rofi -dmenu "${dmenuconfig[@]}" <<< "$db"); then
    exit 1
fi

if ! grep -q "$match" <<< "$db"; then
    match=$(grep -im1 "$match" <<< "$db") || exit 2
fi

output=$(cut -d" " -f1 <<< "$match")

for selection in clipboard primary; do
    xargs printf "%s " <<< "$output" | xsel --logfile /dev/null -i --"$selection"
done
