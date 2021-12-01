#!/usr/bin/env bash

# https://discord.com/developers/docs/resources/emoji
# https://discord.com/developers/docs/topics/rate-limits

set -e

pushd $(git rev-parse --show-toplevel) >/dev/null

TOKEN=
GUILD_ID=
DISCORD_EMOJIS_URL=https://discordapp.com/api/guilds/${GUILD_ID}/emojis
EMOTE_ASSETS=assets/emotes.json
STATIC_EMOTES_PATH=assets/emotes/emoticons/4x
ANIMATED_EMOTES_PATH=assets/emotes/emoticons-animated
AUTH_HEADER="Authorization: Bot ${TOKEN}"

function track {
	last_command=$current_command
	current_command=$BASH_COMMAND
}
function cleanup {
	rm -rf ${TMP_FILE}
}

trap track DEBUG
trap cleanup EXIT

TMP_FILE=$(mktemp)
HTTP_CODE=$(curl -s -w "%{http_code}" -o ${TMP_FILE} -H "${AUTH_HEADER}" ${DISCORD_EMOJIS_URL})
if [[ "${HTTP_CODE}" == *"40"* ]]; then
	echo "failed to request discord emote manifest"
	exit 1
fi

for EMOTE in $(jq -rc '.default[]' ${EMOTE_ASSETS}); do

	# check if emote exists already on discord, if so delet
	EMOTE_ID=$(jq -r '.[] | select(.name == '\"$EMOTE\"') | .id' ${TMP_FILE})
	if [[ -n ${EMOTE_ID} ]]; then
		echo "${EMOTE} already exists.. deleting it"
		curl -s -X DELETE -H "${AUTH_HEADER}" ${DISCORD_EMOJIS_URL}/${EMOTE_ID}
	fi

	EMOTE_PATH=${STATIC_EMOTES_PATH}/${EMOTE}.png
	if [[ ! -f ${EMOTE_PATH} ]]; then
		# TODO: add support for gif emotes
		echo "Skipping ${EMOTE}, ${EMOTE_PATH} does not exist"
		continue
	fi

	FILENAME=$(basename ${EMOTE_PATH})
	NAME=${FILENAME%.*}
	EXTENSION="${FILENAME##*.}"
	DATA="data:image/${EXTENSION};base64,$(base64 -w 0 ${EMOTE_PATH})"

	echo "Uploading ${EMOTE}"

	HTTP_CODE=$(curl -s -w "%{http_code}" -X POST \
		-H "${AUTH_HEADER}" -H "Content-Type: application/json" \
		-d '{"name":"'${NAME}'", "image":"'${DATA}'"}' \
		${DISCORD_EMOJIS_URL})
	if [[ "${HTTP_CODE}" == "429" ]]; then
		echo "Rate limit hit"
		exit 1
	fi
done

popd >/dev/null
